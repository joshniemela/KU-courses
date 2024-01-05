use anyhow::Result;
use axum::http::{Request, StatusCode};
use axum::extract::Query;
use axum::response::{Response, IntoResponse};
use axum::routing::get;
use axum::{Json, Router};
use fastembed::{Embedding, EmbeddingBase, EmbeddingModel, FlagEmbedding, InitOptions};
use nanohtml2text::html2text;
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

const DATA_DIR: &str = "../../../data/new_json";

#[derive(Debug, Deserialize, Clone)]
struct Description {
    content: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Document {
    title: String,
    info: Info,
    description: Description,
}

#[derive(Debug, Deserialize, Clone)]
struct Info {
    id: String,
}

#[derive(Debug, Deserialize, Clone)]
struct EmbeddedDocument {
    id: String,
    title: String,
    title_embedding: Embedding,
    content_embedding: Embedding,
}

fn read_json(path: &Path) -> Result<Document> {
    // TODO: this entire thing is awful, please rewrite
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut doc: Document = serde_json::from_reader(reader)?;
    doc.description.content = html2text(&doc.description.content);
    doc.description.content = doc.description.content.replace("\n", " ");
    doc.description.content = doc.description.content.replace("\t", " ");
    doc.description.content = doc.description.content.replace("\r", " ");
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

fn embed_documents(
    documents: Vec<Document>,
    model: &FlagEmbedding,
) -> Result<Vec<EmbeddedDocument>> {
    let ids: Vec<String> = documents.iter().map(|x| x.info.id.clone()).collect();
    let titles: Vec<String> = documents.iter().map(|x| x.title.clone()).collect();
    let descriptions: Vec<String> = documents
        .iter()
        .map(|x| x.description.content.clone())
        .collect();
    let batch_size = Some(32);
    let embdded_titles = model.passage_embed(titles.clone(), batch_size)?;
    let embdded_descriptions = model.passage_embed(descriptions, batch_size)?;
    let mut embedded_documents: Vec<EmbeddedDocument> = Vec::new();
    for i in 0..documents.len() {
        let embedded_document = EmbeddedDocument {
            id: ids[i].clone(),
            title: titles[i].clone(),
            title_embedding: embdded_titles[i].clone(),
            content_embedding: embdded_descriptions[i].clone(),
        };
        embedded_documents.push(embedded_document);
    }
    Ok(embedded_documents)
}

fn dot_product(a: &Embedding, b: &Embedding) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchQuery {
    query: String,
}

#[tokio::main]
async fn main() {
    let path = Path::new(DATA_DIR);
    let documents = read_jsons(path).unwrap();
    // let documents be the first 100
    //let documents = documents[0..11].to_vec();
    // With default InitOptions
    let model: FlagEmbedding = FlagEmbedding::try_new(InitOptions {
        //model_name: EmbeddingModel::MLE5Large,
        model_name: EmbeddingModel::AllMiniLML6V2,
        show_download_message: true,
        ..Default::default()
    }).unwrap();
    // Embed the documents
    let start = std::time::Instant::now();
    let embedded_documents = embed_documents(documents, &model).unwrap();
    let duration = start.elapsed();
    println!("Time elapsed in embedding documents: {:?}", duration);

    let app = Router::new()
        .route("/health", get(|| async { "healthy" }))
        // search takes a query param "query" and returns a list of the 150 most similar documents
        .route("/search", get(
            |Query(query): Query<SearchQuery>| async move {
                let titles = titles_by_similarity(&query.query, &embedded_documents);
                let response = Json(titles);
                (StatusCode::OK, response)
            },
        )
        );
    let addr = "localhost";
    let port = 4000;
    let listener = tokio::net::TcpListener::bind(&format!("{}:{}", addr, port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
fn titles_by_similarity(
    query: &str,
    embedded_documents: &Vec<EmbeddedDocument>,
) -> Vec<String> {
    let model: FlagEmbedding = FlagEmbedding::try_new(InitOptions {
        //model_name: EmbeddingModel::MLE5Large,
        model_name: EmbeddingModel::AllMiniLML6V2,
        show_download_message: true,
        ..Default::default()
    }).unwrap();
    let query_embedding = model.query_embed(query).unwrap();
    let mut similarities: Vec<(String, f32)> = embedded_documents
        .iter()
        .map(|x| {
            (
                x.title.clone(),
                dot_product(&x.content_embedding, &query_embedding),
            )
        })
        .collect();
    similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    similarities[0..150]
        .iter()
        .map(|x| x.0.clone())
        .collect()
}

