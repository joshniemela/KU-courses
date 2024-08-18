use super::{Coordinator, Course};
use crate::embedding::{CoordinatorEmbedding, CourseEmbedding};
use crate::populate::Document;
use anyhow::Result;
use pgvector::Vector;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::{query, Row};

pub struct PostgresDB {
    pub pool: PgPool,
}
impl PostgresDB {
    pub async fn new(db_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(3)
            .connect(db_url)
            .await?;
        Ok(Self { pool })
    }

    /// Returns all the course ids that have outdated or non-existent embeddings
    /// This is computed by checking if the course modified timestamp is greater than the last modified
    /// timestamp of the title embedding or the content embedding
    pub async fn get_outdated_embedding_course_ids(&self) -> Result<Vec<String>> {
        let result = query!(
            "SELECT c.id
            FROM course c
            LEFT JOIN title_embedding te ON c.id = te.course_id
            LEFT JOIN content_embedding ce ON c.id = ce.course_id
            WHERE
                c.last_modified > COALESCE(te.last_modified, to_timestamp(0)) OR
                c.last_modified > COALESCE(ce.last_modified, to_timestamp(0))"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut ids: Vec<String> = Vec::new();
        for row in result {
            ids.push(row.id.expect("id"))
        }

        Ok(ids)
    }

    /// Returns all the coordinators in the Vec of coordinator emails that do not have an embedding
    pub async fn get_missing_embedding_email_names(&self) -> Result<Vec<Coordinator>> {
        // Due to a weird bug, this has to not be a macroo query
        let result = query(
            "SELECT coordinator.email, coordinator.full_name
            FROM coordinator
            LEFT JOIN name_embedding ne ON coordinator.email = ne.email
            WHERE ne.embedding IS NULL",
        )
        .fetch_all(&self.pool)
        .await?;

        let mut coordinators = Vec::new();
        for row in result {
            coordinators.push(Coordinator {
                email: row.try_get("email")?,
                name: row.try_get("full_name")?,
            });
        }

        Ok(coordinators)
    }

    /// Returns all the courses in the Vec of course ids
    pub async fn get_courses_by_ids(&self, ids: &[String]) -> Result<Vec<Course>> {
        let mut courses = Vec::new();

        let result = query!(
            "SELECT id, title, content FROM course WHERE id = ANY($1)",
            &ids
        )
        .fetch_all(&self.pool)
        .await?;

        for row in result {
            let course = Course {
                id: row.id,
                title: row.title,
                content: row.content,
            };
            courses.push(course);
        }

        Ok(courses)
    }

    /// Inserts the document into the database
    /// If the document already exists, it updates the title, content, and last_modified timestamp
    /// This is used by populate.rs but is not strictly required
    /// for the search functionality
    /// TODO: all insertion functionality should be moved out of this service
    pub async fn upsert_document(&self, document: &Document) -> Result<()> {
        // start by checking if the Document is the same as the one in the database
        // if it is, do nothing
        let result = query!(
            "SELECT title, content FROM course WHERE id = $1",
            document.info.id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = result {
            if row.title == document.title && row.content == document.description.content {
                return Ok(());
            }
        }

        let mut tx = self.pool.begin().await?;

        query!(
            "INSERT INTO course (id, title, content) VALUES ($1, $2, $3)
             ON CONFLICT(id) DO UPDATE SET title = $2, content = $3, last_modified = CURRENT_TIMESTAMP",
            document.info.id,
            document.title,
            document.description.content
        ).execute(&mut *tx).await?;

        // A coordinator may have been removed, so we need to delete all coordinators for this course
        query!(
            "DELETE FROM course_coordinator WHERE course_id = $1",
            document.info.id
        )
        .execute(&mut *tx)
        .await?;

        // no conflict, if the coordinator exists do nothing
        for coordinator in document.logistics.coordinators.iter() {
            query!(
                "INSERT INTO coordinator (email, full_name) VALUES ($1, $2)
                 ON CONFLICT(email) DO NOTHING",
                coordinator.email,
                coordinator.name
            )
            .execute(&mut *tx)
            .await?;

            query!(
                "INSERT INTO course_coordinator (course_id, email) VALUES ($1, $2)",
                document.info.id,
                coordinator.email
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    /// Inserts the coordinator embedding into the database
    /// If the coordinator already exists, it does nothing,
    /// this is because we assume the names of the coordinators are immutable
    pub async fn insert_coordinator_embedding(
        &self,
        coordinator: CoordinatorEmbedding,
    ) -> Result<()> {
        query(
            "INSERT INTO name_embedding (email, embedding) VALUES ($1, $2)
            ON CONFLICT(email) DO NOTHING",
        )
        .bind(coordinator.email)
        .bind(Vector::from(coordinator.name.to_owned()))
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Inserts the course embedding into the database
    /// If the course already exists, it updates the embedding and the last_modified timestamp
    pub async fn insert_course_embedding(&self, course_embedding: CourseEmbedding) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        query(
            "INSERT INTO title_embedding (course_id, embedding) VALUES ($1, $2)
            ON CONFLICT(course_id) DO UPDATE SET embedding = $2, last_modified = CURRENT_TIMESTAMP",
        )
        .bind(&course_embedding.id)
        .bind(Vector::from(course_embedding.title.to_owned()))
        .execute(&mut *tx)
        .await?;

        query(
            "INSERT INTO content_embedding (course_id, embedding) VALUES ($1, $2)
            ON CONFLICT(course_id) DO UPDATE SET embedding = $2, last_modified = CURRENT_TIMESTAMP",
        )
        .bind(course_embedding.id)
        .bind(Vector::from(course_embedding.content.to_owned()))
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    /// Returns the most relevant course ids based on the query embedding
    /// The title embedding is the title for that course
    /// The content embedding is the content for that course
    /// The coordinator embedding for each course is the most relevant coordinator for that course,
    /// if the coordinator's distance is greater than 0.8, it is clipped to 0.9, if it is less then it is halved
    /// to give it more importance in the total distance
    /// The relevance is then computed as the sum of the distances between the query embedding and the
    /// title embedding, content embedding, and coordinator embedding
    /// and is returned in ascending order (lower is better)
    pub async fn get_most_relevant_course_ids(
        &self,
        query_embedding: &[f32],
    ) -> Result<Vec<String>> {
        let result = query("
WITH
title_search AS (
SELECT
    course_id, embedding <-> $1 AS distance
FROM
    title_embedding
),

content_search AS (
SELECT
    course_id, embedding <-> $1 AS distance
FROM
    content_embedding
),

coordinator_search AS (
SELECT
    course_id, MIN(
        CASE
            WHEN embedding <-> $1 > 0.8 THEN 0.9
            ELSE (embedding <-> $1) / 2
        END) AS distance
FROM
    course_coordinator
    INNER JOIN
        name_embedding
    ON
        course_coordinator.email = name_embedding.email
GROUP BY course_id),



combined_search AS (
    SELECT
        course.id,
        title_search.distance + content_search.distance + coordinator_search.distance AS total_distance
    FROM
        title_search
    INNER JOIN
        content_search ON title_search.course_id = content_search.course_id
    INNER JOIN
        coordinator_search ON title_search.course_id = coordinator_search.course_id
    INNER JOIN
        course ON title_search.course_id = course.id
),

ranked_courses AS (
    SELECT
        id,
        total_distance,
        ROW_NUMBER() OVER (PARTITION BY id ORDER BY total_distance) AS rn
    FROM
        combined_search
)

SELECT
    id
FROM
    ranked_courses
WHERE
    rn = 1
ORDER BY
    total_distance
LIMIT 200;
")
        .bind(Vector::from(query_embedding.to_owned()))
        .fetch_all(&self.pool)
        .await?;
        let mut ids: Vec<String> = Vec::new();
        for row in result {
            ids.push(row.try_get("id")?);
        }

        Ok(ids)
    }
}
