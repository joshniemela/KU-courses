use super::{Coordinator, Document, EmbeddedDocument, Embedding};
use anyhow::Result;
use rusqlite::{params, Connection};
use serde_json::json;

pub fn initialise_db(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;
    // This is the raw table responsible for storing the documents
    tx.execute(
        "CREATE TABLE IF NOT EXISTS documents (
            id TEXT PRIMARY KEY,
            title TEXT,
            content TEXT,
            lastmodified INTEGER DEFAULT (strftime('%s', 'now'))
        )",
        params![],
    )?;

    // This table stores the coordinators, this is needed since we have a many-to-many relationship
    tx.execute(
        "CREATE TABLE IF NOT EXISTS coordinators (
            email TEXT PRIMARY KEY,
            name TEXT
        )",
        params![],
    )?;

    // This is the link between course coordinator's email and the course id
    tx.execute(
        "CREATE TABLE IF NOT EXISTS course_coordinators (
            course_id TEXT,
            coordinator_email TEXT,
            PRIMARY KEY (course_id, coordinator_email),
            FOREIGN KEY (course_id) REFERENCES documents(id),
            FOREIGN KEY (coordinator_email) REFERENCES coordinators(email)
        )",
        params![],
    )?;

    // This stores the coordinators as binary objects since they're vector embeddings
    tx.execute(
        "CREATE TABLE IF NOT EXISTS embeddings (
            id TEXT PRIMARY KEY UNIQUE,
            title_embedding BLOB,
            content_embedding BLOB,
            coordinator_embeddings BLOB
        )",
        params![],
    )?;
    tx.commit()?;
    Ok(())
}

pub fn get_documents_by_ids(conn: &Connection, ids: &[String]) -> Result<Vec<Document>> {
    let mut stmt = conn.prepare("SELECT id, title, content FROM documents WHERE id = ?")?;
    // first create a bunch of empty documents
    let mut documents: Vec<Document> = ids.iter().map(|_| Document::default()).collect();
    for (i, id) in ids.iter().enumerate() {
        let mut rows = stmt.query(params![id])?;
        let row = rows.next()?.unwrap();
        let title: String = row.get(1)?;
        let content: String = row.get(2)?;
        documents[i].info.id = id.clone();
        documents[i].title = title;
        documents[i].description.content = content;
    }
    for document in &mut documents {
        // Find all the coordinators that are associated with this course,
        // we want the name, the emails are stored in the course_coordinators table and need ot be joined with
        // the coordinators table and documents table
        let mut stmt = conn.prepare(
            "SELECT name FROM coordinators INNER JOIN course_coordinators ON coordinators.email = course_coordinators.coordinator_email WHERE course_coordinators.course_id = ?")?;
        let mut rows = stmt.query(params![document.info.id])?;
        while let Some(row) = rows.next()? {
            let coordinator_name: String = row.get(0)?;
            document.logistics.coordinators.push(Coordinator {
                name: coordinator_name,
            });
        }
    }

    Ok(documents)
}

pub fn upsert_document(conn: &mut Connection, document: &Document) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute(
        "INSERT INTO documents (id, title, content) VALUES (?1, ?2, ?3)
        ON CONFLICT(id) DO UPDATE SET title = ?2, content = ?3, lastmodified = strftime('%s', 'now')",
        params![document.info.id, document.title, document.description.content],
    )?;

    // Modifying a document means the embeddings are no longer valid
    tx.execute(
        "DELETE FROM embeddings WHERE id = ?1",
        params![document.info.id],
    )?;
    // A coordinator may have been removed, so we need to delete all coordinators for this course
    tx.execute(
        "DELETE FROM course_coordinators WHERE course_id = ?1",
        params![document.info.id],
    )?;

    // no conflict, if the coordinator exists do nothing
    for coordinator in document.logistics.coordinators.iter() {
        tx.execute(
            "INSERT INTO coordinators (email, name) VALUES (?1, ?2)
            ON CONFLICT(email) DO NOTHING",
            params![coordinator.name, coordinator.name],
        )?;

        tx.execute(
            "INSERT INTO course_coordinators (course_id, coordinator_email) VALUES (?1, ?2)
            ON CONFLICT(course_id, coordinator_email) DO NOTHING",
            params![document.info.id, coordinator.name],
        )?;
    }
    tx.commit()?;
    Ok(())
}

pub fn get_document_mod_times(conn: &Connection) -> Result<Vec<(String, i64)>> {
    let mut stmt = conn
        .prepare("SELECT id, lastmodified FROM documents")
        .unwrap();
    let mut documents = Vec::new();
    let documents_iter = stmt
        .query_map(params![], |row| {
            let id: String = row.get(0)?;
            let lastmodified: i64 = row.get(1)?;
            Ok((id, lastmodified))
        })
        .unwrap();
    for document in documents_iter {
        let document = document.unwrap();
        documents.push(document);
    }
    Ok(documents)
}

pub fn documents_wo_embeddings(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn
        .prepare("SELECT id FROM documents WHERE id NOT IN (SELECT id FROM embeddings)")
        .unwrap();
    let mut documents = Vec::new();
    let documents_iter = stmt
        .query_map(params![], |row| {
            let id: String = row.get(0)?;
            Ok(id)
        })
        .unwrap();
    for document in documents_iter {
        let document = document.unwrap();
        documents.push(document);
    }
    Ok(documents)
}

pub fn insert_embeddings(conn: &mut Connection, embeddings: &EmbeddedDocument) -> Result<()> {
    conn.execute(
        "INSERT INTO embeddings (id, title_embedding, content_embedding, coordinator_embeddings) VALUES (?1, ?2, ?3, ?4)",
        params![embeddings.id,
                json![embeddings.title_embedding],
                json![embeddings.content_embedding],
                json![embeddings.coordinator_embeddings]],
    )?;
    Ok(())
}

pub fn get_all_embeddings(conn: &Connection) -> Result<Vec<EmbeddedDocument>> {
    let mut stmt = conn.prepare(
        "SELECT id, title_embedding, content_embedding, coordinator_embeddings FROM embeddings",
    )?;
    let mut embedded_documents = Vec::new();
    let embedded_documents_iter = stmt.query_map(params![], |row| {
        let id: String = row.get(0)?;
        let title_embedding: String = row.get(1)?;
        let content_embedding: String = row.get(2)?;
        let coordinator_embeddings: String = row.get(3)?;
        Ok((
            id,
            title_embedding,
            content_embedding,
            coordinator_embeddings,
        ))
    })?;
    for embedded_document in embedded_documents_iter {
        let embedded_document = embedded_document?;
        let title_embedding: Embedding = serde_json::from_str(&embedded_document.1)?;
        let content_embedding: Embedding = serde_json::from_str(&embedded_document.2)?;
        let coordinator_embeddings: Vec<Embedding> = serde_json::from_str(&embedded_document.3)?;
        let embedded_document = EmbeddedDocument {
            id: embedded_document.0,
            title_embedding,
            content_embedding,
            coordinator_embeddings,
        };
        embedded_documents.push(embedded_document);
    }
    Ok(embedded_documents)
}
