use super::{Coordinator, Document, EmbeddedDocument};
use anyhow::Result;
use pgvector::Vector;
use sqlx::postgres::{PgPool, PgPoolOptions, Postgres};
use sqlx::Row;
use sqlx::{query, Transaction};

#[allow(dead_code)]
type PgTransaction<'a> = Transaction<'a, Postgres>;

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

    pub async fn get_outdated_embedding_course_ids(&self) -> Result<Vec<String>> {
        let result = query!(
            "SELECT c.id
            FROM course c
            LEFT JOIN title_embedding te ON c.id = te.course_id
            LEFT JOIN content_embedding ce ON c.id = ce.course_id
            WHERE
                c.last_modified > COALESCE(te.last_modified, to_timestamp(0)) OR
                c.last_modified > COALESCE(ce.last_modified, to_timestamp(0))"
        ).fetch_all(&self.pool).await?;

        let mut ids: Vec<String> = Vec::new();
        for row in result {
            ids.push(row.id.expect("id should be present"));
        }

        Ok(ids)
    }

    pub async fn get_missing_embedding_email_names(&self) -> Result<Vec<(String, String)>> {
        let result = query!(
            "SELECT coordinator.email, coordinator.full_name
            FROM coordinator
            LEFT JOIN name_embedding ne ON coordinator.email = ne.email
            WHERE ne.embedding IS NULL"
        ).fetch_all(&self.pool).await?;

        let mut coordinators = Vec::new();
        for row in result {
            coordinators.push((row.email, row.full_name));
        }

        Ok(coordinators)
    }

    pub async fn get_documents_by_ids(&self, ids: &[String]) -> Result<Vec<Document>> {
        let mut documents = Vec::new();

        let result = query!(
            "SELECT id, title, content FROM course WHERE id = ANY($1)",
            &ids
        )
        .fetch_all(&self.pool)
        .await?;

        // We make one document and then we reuse it
        let mut document = Document::default();

        for row in result {
            // We find the coordinators for this course
            let coordinators: Vec<Coordinator> = query!(
                "SELECT full_name, coordinator.email FROM coordinator INNER JOIN course_coordinator
                 ON coordinator.email = course_coordinator.email
                 WHERE course_coordinator.course_id = $1",
                row.id
            )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|row| Coordinator {
                name: row.full_name,
                email: row.email,
            })
            .collect();

            document.info.id = row.id;
            document.title = row.title;
            document.description.content = row.content;
            document.logistics.coordinators = coordinators;

            documents.push(document.clone());
        }

        Ok(documents)
    }

   pub async fn upsert_document(&self, document: &Document) -> Result<()> {
        // start by checking if the Document is the same as the one in the database
        // if it is, do nothing

        let result = query!(
            "SELECT title, content FROM course WHERE id = $1",
            document.info.id
        ).fetch_optional(&self.pool).await?;

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

    pub async fn insert_coordinator_embedding(&self, email: &str, embedding: &[f32]) -> Result<()> {
        query(
            "INSERT INTO name_embedding (email, embedding) VALUES ($1, $2)
            ON CONFLICT(email) DO NOTHING")
            .bind(email)
            .bind(Vector::from(embedding.to_owned()))
            .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn insert_course_embeddings(&self, course_id: &str, title_embedding: &[f32], content_embedding: &[f32]) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        query(
            "INSERT INTO title_embedding (course_id, embedding) VALUES ($1, $2)
            ON CONFLICT(course_id) DO UPDATE SET embedding = $2, last_modified = CURRENT_TIMESTAMP")
            .bind(course_id)
            .bind(Vector::from(title_embedding.to_owned()))
            .execute(&mut *tx).await?;

        query(
            "INSERT INTO content_embedding (course_id, embedding) VALUES ($1, $2)
            ON CONFLICT(course_id) DO UPDATE SET embedding = $2, last_modified = CURRENT_TIMESTAMP")
            .bind(course_id)
            .bind(Vector::from(content_embedding.to_owned()))
            .execute(&mut *tx).await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn get_name_similarities(
        &self,
        query_embedding: &[f32],
    ) -> Result<Vec<(String, f64)>> {
        let result = query(
            "SELECT full_name, embedding <-> $1 AS distance
             FROM name_embedding NATURAL JOIN coordinator
             ORDER BY distance ASC
             LIMIT 100",
        )
        .bind(Vector::from(query_embedding.to_owned()))
        .fetch_all(&self.pool)
        .await?;
        let mut names: Vec<(String, f64)> = Vec::new();
        for row in result {
            names.push((row.try_get("full_name")?, row.try_get("distance")?));
        }

        Ok(names)
    }

    pub async fn get_title_similarities(
        &self,
        query_embedding: &[f32],
    ) -> Result<Vec<(String, f64)>> {
        let result = query(
            "SELECT title, embedding <-> $1 AS distance
            FROM title_embedding INNER JOIN course ON title_embedding.course_id = course.id
            ORDER BY distance ASC
            LIMIT 100",
        )
        .bind(Vector::from(query_embedding.to_owned()))
        .fetch_all(&self.pool)
        .await?;
        let mut ids: Vec<(String, f64)> = Vec::new();
        for row in result {
            ids.push((row.try_get("title")?, row.try_get("distance")?));
        }

        Ok(ids)
    }

    pub async fn get_content_similarities(&self, query_embedding: &[f32]) -> Result<Vec<f64>> {
        let result = query(
            "SELECT embedding <-> $1 AS distance
            FROM content_embedding
            ORDER BY distance ASC
            LIMIT 100",
        )
        .bind(Vector::from(query_embedding.to_owned()))
        .fetch_all(&self.pool)
        .await?;
        let mut ids: Vec<f64> = Vec::new();
        for row in result {
            ids.push(row.try_get("distance")?);
        }

        Ok(ids)
    }

    pub async fn get_most_relevant_course_ids(
        &self,
        query_embedding: &[f32],
    ) -> Result<Vec<String>> {
        // Find the most relevant coordinator for each course, then find the title and content embedding similarities
        // each course may have multiple coordinators
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
    course_id, COALESCE(MIN(embedding <-> $1), 1000) AS distance
FROM
    course_coordinator
    INNER JOIN
        name_embedding
    ON
        course_coordinator.email = name_embedding.email
GROUP BY course_id)

SELECT
    title
FROM
    title_search
    INNER JOIN
        content_search
    ON title_search.course_id = content_search.course_id
    INNER JOIN coordinator_search
        ON title_search.course_id = coordinator_search.course_id
    INNER JOIN course
        ON title_search.course_id = course.id
ORDER BY title_search.distance^2 + content_search.distance^2 + coordinator_search.distance^2 ASC
LIMIT 200")
        .bind(Vector::from(query_embedding.to_owned()))
        .fetch_all(&self.pool)
        .await?;
        let mut ids: Vec<String> = Vec::new();
        for row in result {
            ids.push(row.try_get("title")?);
        }

        Ok(ids)
    }
}
