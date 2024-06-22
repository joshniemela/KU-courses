use super::{Coordinator, Document, EmbeddedDocument};
use anyhow::Result;
use sqlx::postgres::{PgPoolOptions, Postgres, PgPool};
use sqlx::{Transaction, query};
use sqlx::Row;
use pgvector::Vector;

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


    pub async fn insert_course(&self, document: &Document) -> Result<()> {
        // grab the coordinators
        let coordinators = &document.logistics.coordinators;

        let mut tx = self.pool.begin().await?;

        // export DATABASE_URL=postgres://postgres:password123@localhost:5432/disku
        query!(
            "INSERT INTO course (id, title, content) VALUES ($1, $2, $3)
            ON CONFLICT(id) DO UPDATE SET title = $2, content = $3, last_modified = CURRENT_TIMESTAMP",
            document.info.id,
            document.title,
            document.description.content
        ).execute(&mut *tx).await?;

        // We add coordinators, these are immutable and we just insert if conflict do nothing
        for coordinator in coordinators {
            query!(
                "INSERT INTO coordinator (email, full_name) VALUES ($1, $2)
                ON CONFLICT(email) DO NOTHING",
                coordinator.email,
                coordinator.name
            ).execute(&mut *tx).await?;

            query!(
                "INSERT INTO course_coordinator (id, email) VALUES ($1, $2)
                ON CONFLICT(id, email) DO NOTHING",
                document.info.id,
                coordinator.email
            ).execute(&mut *tx).await?;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn get_documents_by_ids(&self, ids: &[String]) -> Result<Vec<Document>> {
        let mut documents = Vec::new();

        let result = query!(
            "SELECT id, title, content FROM course WHERE id = ANY($1)",
            &ids
        ).fetch_all(&self.pool).await?;

        // We make one document and then we reuse it
        let mut document = Document::default();

        for row in result {
            // We find the coordinators for this course
            let coordinators: Vec<Coordinator> = query!(
                "SELECT full_name, coordinator.email FROM coordinator INNER JOIN course_coordinator
                ON coordinator.email = course_coordinator.email
                WHERE course_coordinator.id = $1",
                row.id
            ).fetch_all(&self.pool).await?
                .into_iter()
                .map(|row| {
                    Coordinator {
                        name: row.full_name,
                        email: row.email,
                    }
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
            "DELETE FROM course_coordinator WHERE id = $1",
            document.info.id
        ).execute(&mut *tx).await?;

        // no conflict, if the coordinator exists do nothing
        for coordinator in document.logistics.coordinators.iter() {
            query!(
                "INSERT INTO coordinator (email, full_name) VALUES ($1, $2)
                ON CONFLICT(email) DO NOTHING",
                coordinator.email,
                coordinator.name
            ).execute(&mut *tx).await?;

            query!(
                "INSERT INTO course_coordinator (id, email) VALUES ($1, $2)
                ON CONFLICT(id, email) DO NOTHING",
                document.info.id,
                coordinator.email
            ).execute(&mut *tx).await?;
        }

        tx.commit().await?;
        Ok(())

    }

    pub async fn get_document_mod_times(&self) -> Result<Vec<(String, i64)>> {
        let mut stamps: Vec<(String, i64)> = Vec::new();
        let result = query!(
            "SELECT id, last_modified FROM course"
        ).fetch_all(&self.pool).await?;

        for row in result {
            stamps.push((row.id, row.last_modified.assume_utc().unix_timestamp()));
        }

        Ok(stamps)
    }

    pub async fn get_all_courses_wo_embeddings(&self) -> Result<Vec<String>> {
        // The courses should have a title_embedding and more than 0 content_embeddings
        // the embeddings are two separate tables
        let result = query!(
            "SELECT course.id
             FROM course
             LEFT JOIN title_embedding ON course.id = title_embedding.course_id
             LEFT JOIN content_embedding ON course.id = content_embedding.course_id
             WHERE title_embedding.course_id IS NULL OR content_embedding.course_id IS NULL"
        ).fetch_all(&self.pool).await?;

        let mut ids: Vec<String> = Vec::new();
        for row in result {
            ids.push(row.id.expect("id should be present"));
        }

        Ok(ids)
    }


    pub async fn insert_embeddings(&self, embeddings: &EmbeddedDocument) -> Result<()> {
        let mut tx = self.pool.begin().await?;


        query(
            "INSERT INTO title_embedding (course_id, embedding) VALUES ($1, $2)
            ON CONFLICT(course_id) DO UPDATE SET embedding = $2")
            .bind(&embeddings.id)
            .bind(Vector::from(embeddings.title_embedding.clone()))
            .execute(&mut *tx).await?;

        query(
            "INSERT INTO content_embedding (course_id, embedding) VALUES ($1, $2)")
            .bind(&embeddings.id)
            .bind(Vector::from(embeddings.content_embedding.clone()))
            .execute(&mut *tx).await?;

        for coordinator in &embeddings.coordinator_embeddings {
            query(
                "INSERT INTO name_embedding (email, embedding) VALUES ($1, $2)
                ON CONFLICT(email) DO UPDATE SET embedding = $2")
                .bind(&coordinator.0)
                .bind(Vector::from(coordinator.1.clone()))
                .execute(&mut *tx).await?;
        }


        tx.commit().await?;
        Ok(())
    }


    pub async fn get_most_relevant_course_ids(&self, query_embedding: &Vec<f32>) -> Result<Vec<String>> {
        let result = query(
            "SELECT course_id, title_embedding.embedding <-> $1 AS distance
            FROM title_embedding
            ORDER BY distance ASC
            LIMIT 100"
        )
            .bind(Vector::from(query_embedding.clone()))
            .fetch_all(&self.pool).await?;
        let mut ids: Vec<String> = Vec::new();
        for row in result {
            ids.push(row.try_get("course_id")?);
        }

        Ok(ids)
    }
}

/*
    // We will use reciprocal rank fusion to combine the title, name and content embeddings
    pub async fn get_most_relevant_course_ids(&self, query_embedding: &Vec<f32>) -> Result<Vec<String>> {
        let result = query(
            "WITH title_search AS (
                SELECT id, RANK () OVER (ORDER BY embedding <=> %(query_embedding)s) AS rank
                FROM title_embedding
                ORDER BY embedding <=> %(query_embedding)s
            ),
            content_search AS (
                SELECT id, RANK () OVER (ORDER BY embedding <=> %(query_embedding)s) AS rank
                FROM content_embedding
                ORDER BY embedding <=> %(query_embedding)s
            )
            SELECT
COALESCE(title_search.id, content_search.id) AS id.
COALESCE(1.0 / (%(k)s + title_search.rank), 0) +
COALESCE(1.0 / (%(k)s + content_search.rank), 0) AS score
FROM title_search
FULL OUTER JOIN content_search ON title_search.id = content_search.id
ORDER BY score DESC
LIMIT 100"
        )
            .bind(query_embedding)
            .bind(50)
            .fetch_all(&self.pool).await?;

        let mut ids: Vec<String> = Vec::new();
        for row in result {
            ids.push(row.id.expect("id should be present"));
        }

        Ok(ids)
    }
}
*/



// Find the most relevant courses compared to the course ID NDAK22000U only using title similarity, do this in raw SQL
// SELECT title, RANK() OVER (ORDER BY embedding <=> (SELECT embedding FROM title_embedding WHERE course_id = 'NDAK22000U')) AS rank
// FROM title_embedding INNER JOIN course ON title_embedding.course_id = course.id
