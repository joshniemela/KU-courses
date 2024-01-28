use super::{Coordinator, Document};
use anyhow::Result;
use rusqlite::{params, Connection};

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
