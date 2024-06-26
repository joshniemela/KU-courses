use super::{PostgresDB, Coordinator};
use anyhow::Result;
use nanohtml2text::html2text;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Logistics {
    pub coordinators: Vec<Coordinator>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Description {
    pub content: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Document {
    pub title: String,
    pub info: Info,
    pub description: Description,
    pub logistics: Logistics,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Info {
    pub id: String,
}

fn read_json(path: &Path) -> Result<Document> {
    // TODO: this entire thing is awful, please rewrite
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut doc: Document = serde_json::from_reader(reader)?;
    doc.description.content = html2text(&doc.description.content);
    doc.description.content = doc.description.content.replace('\n', " ");
    doc.description.content = doc.description.content.replace('\t', " ");
    doc.description.content = doc.description.content.replace('\r', " ");
    Ok(doc)
}

fn read_jsons(path: &Path) -> Result<Vec<Document>> {
    // this should read all the jsons in the directory
    let file_names = std::fs::read_dir(path)?;
    let mut documents = Vec::new();
    for file_name in file_names {
        let file_name = file_name?;
        let path = file_name.path();
        let document = read_json(&path)?;
        documents.push(document);
    }
    Ok(documents)
}

pub async fn upsert_documents_from_path(db: &PostgresDB, path: &Path) -> Result<()> {
    let documents = read_jsons(path)?;
    for document in documents {
        db.upsert_document(&document).await?;
    }
    Ok(())
}
