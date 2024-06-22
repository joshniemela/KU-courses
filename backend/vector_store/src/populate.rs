use super::Document;
use anyhow::Result;
use nanohtml2text::html2text;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

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

pub fn read_jsons(path: &Path) -> Result<Vec<Document>> {
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
