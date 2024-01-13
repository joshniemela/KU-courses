use anyhow::Result;
use std::env;
use axum::extract::Query;
use axum::routing::get;
use axum::extract::State;
use axum::{Json, Router};
use fastembed::{Embedding, EmbeddingBase, EmbeddingModel, FlagEmbedding, InitOptions};
use nanohtml2text::html2text;
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use rayon::prelude::*;

#[derive(Debug, Deserialize, Clone)]
struct Coordinator {
    name: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Logistics {
    coordinators: Vec<Coordinator>,
}

#[derive(Debug, Deserialize, Clone)]
struct Description {
    content: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Document {
    title: String,
    info: Info,
    description: Description,
    logistics: Logistics,
}

#[derive(Debug, Deserialize, Clone)]
struct Info {
    id: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
struct EmbeddedDocument {
    id: String,
    title_embedding: Embedding,
    content_embedding: Embedding,
    coordinator_embeddings: Vec<Embedding>,
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
    let ids: Vec<String> = documents.par_iter().map(|x| x.info.id.clone()).collect();
    let titles: Vec<String> = documents.par_iter().map(|x| x.title.clone()).collect();
    let descriptions: Vec<String> = documents
        .par_iter()
        .map(|x| x.description.content.clone())
        .collect();
    let coordinators: Vec<Vec<Coordinator>> = documents
        .par_iter()
        .map(|x| x.logistics.coordinators.clone())
        .collect();
    let batch_size = Some(32);
    let embdded_titles = model.passage_embed(titles.clone(), batch_size)?;
    let embdded_descriptions = model.passage_embed(descriptions, batch_size)?;
    let embedded_coordinators: Vec<Vec<Embedding>> = coordinators
        .par_iter()
        .map(|x| {
            let coordinator_names: Vec<String> = x.par_iter().map(|x| x.name.clone()).collect();
            model.passage_embed(coordinator_names, batch_size).unwrap()
        })
        .collect();
    let mut embedded_documents: Vec<EmbeddedDocument> = Vec::new();
    for i in 0..documents.len() {
        let embedded_document = EmbeddedDocument {
            id: ids[i].clone(),
            title_embedding: embdded_titles[i].clone(),
            content_embedding: embdded_descriptions[i].clone(),
            coordinator_embeddings: embedded_coordinators[i].clone(),
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


#[derive(Clone)]
struct AppState {
    embedded_documents: Vec<EmbeddedDocument>,
    model_ref: &'static FlagEmbedding,
}

fn make_embedding_model() -> Result<FlagEmbedding> {
    let model: FlagEmbedding = FlagEmbedding::try_new(InitOptions {
        //model_name: EmbeddingModel::MLE5Large,
        model_name: EmbeddingModel::AllMiniLML6V2,
        show_download_message: true,
        ..Default::default()
    })?;
    Ok(model)
}

#[tokio::main]
async fn main() {
    let data_dir = env::var("DATA_DIR").expect("DATA_DIR not set");
    let new_json_dir = data_dir.to_owned() + "new_json/";
    let embedded_documents_path = data_dir.to_owned() + "embedded_documents.json";
    let path = Path::new(&new_json_dir);
    let documents = read_jsons(path).unwrap();

    let model = make_embedding_model().unwrap();

    let embedded_documents_path = Path::new(&embedded_documents_path);
    // temporarily disabled caching becasue it wont update on new courses
    let embedded_documents = if false {
        println!("Reading from {}", embedded_documents_path.display());
        let file = File::open(embedded_documents_path).unwrap();
        let reader = BufReader::new(file);
        let embedded_documents: Vec<EmbeddedDocument> = serde_json::from_reader(reader).unwrap();
        println!("Read from {}", embedded_documents_path.display());
        embedded_documents
    } else {
        println!("Writing to {}", embedded_documents_path.display());
        let embedded_documents = embed_documents(documents, &model).unwrap();
        // ORT doesn't release the memory it uses, so we need to drop the model
        drop(model);
        // also disabled caching here
        //let file = File::create(embedded_documents_path).unwrap();
        //let writer = BufWriter::new(file);
        //serde_json::to_writer(writer, &embedded_documents).unwrap();
        println!("Wrote to {}", embedded_documents_path.display());
        embedded_documents
    };




    // Recreate the model since we just killed it
    let model = make_embedding_model().unwrap();

    let state = AppState {
        embedded_documents: embedded_documents.clone(),
        model_ref: Box::leak(Box::new(model)),
    };

    let app = Router::new()
        .route("/health", get(|| async { "healthy" }))
        // search takes a query param "query" and returns a list of the 150 most similar documents
        .route("/search", get(search))
        .with_state(state);
    let addr = env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS must be set");
    let port = env::var("SERVER_PORT").expect("SERVER_PORT must be set");
    let listener = tokio::net::TcpListener::bind(&format!("{}:{}", addr, port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn document_similarity(
    query_embedding: &Embedding,
    embedded_document: &EmbeddedDocument,
) -> f32 {
    let content_similarity = dot_product(&query_embedding, &embedded_document.content_embedding);

    // We weigh by 1.25 because we want the title to be more important than the content
    let title_similarity = dot_product(&query_embedding, &embedded_document.title_embedding);
    let best_coordinator_similarity = embedded_document
        .coordinator_embeddings
        .iter()
        .map(|x| dot_product(&query_embedding, x))
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
        // if coordinator_similarity is less than 0.5, we set it to 0
    let coordinator_similarity = if best_coordinator_similarity < 0.5 {
        0.0
    } else {
        best_coordinator_similarity
    };

    // grab the highest of the three similarities
    content_similarity.max(title_similarity).max(coordinator_similarity)
}

fn ids_by_similarity(
    query: &str,
    embedded_documents: &Vec<EmbeddedDocument>,
    model: &FlagEmbedding,
) -> Vec<String> {
    let query_embedding = model.query_embed(query).unwrap();
    let mut similarities: Vec<(String, f32)> = embedded_documents
        .par_iter()
        .map(|x| {
            (
                x.id.clone(),
                document_similarity(&query_embedding, &x),
            )
        })
        .collect();
    similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    similarities[0..150]
        .par_iter()
        .map(|x| x.0.clone())
        .collect()
}


async fn search(
    Query(query): Query<SearchQuery>,
    State(state): State<AppState>,
) -> Json<Vec<String>> {
    let query = query.query;
    let model = state.model_ref;
    let ids = ids_by_similarity(&query, &state.embedded_documents, model);
    Json(ids)
}
