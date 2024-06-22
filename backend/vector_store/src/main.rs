use anyhow::Result;
use std::env;
use axum::extract::Query;
use axum::routing::get;
use axum::extract::State;
use axum::{Json, Router};
use fastembed::{Embedding, EmbeddingModel, TextEmbedding, InitOptions};
use serde::Deserialize;
use serde::Serialize;
use std::fs::metadata;
use std::path::Path;
use rayon::prelude::*;
use std::sync::Arc;

use sqlx::migrate;

mod db;
use db::PostgresDB;

mod populate;
use populate::read_jsons;

#[derive(Debug, Deserialize, Clone)]
struct Coordinator {
    name: String,
    email: String,
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
impl Document {
    fn default() -> Self {
        Document {
            title: String::new(),
            info: Info { id: String::new() },
            description: Description {
                content: String::new(),
            },
            logistics: Logistics {
                coordinators: Vec::new(),
            },
        }
    }
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
    coordinator_embeddings: Vec<(String, Embedding)>,
}

fn passage_embed(
    passages: Vec<String>, model: &TextEmbedding, batch_size: Option<usize>) -> Result<Vec<Embedding>> {
    // for each passage, add passage: to the front of it
    let passages: Vec<String> = passages
        .par_iter()
        .map(|x| format!("passage: {}", x))
        .collect();
    model.embed(passages, batch_size)
}

fn query_embed(query: &str, model: &TextEmbedding) -> Result<Embedding> {
    // add query: to the front of the query
    model.embed(vec![format!("query: {}", query)], None).map(|x| x[0].clone())
}

fn embed_documents(
    documents: Vec<Document>,
    model: &TextEmbedding,
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
    let embdded_titles = passage_embed(titles.clone(), &model, batch_size)?;
    let embdded_descriptions = passage_embed(descriptions, &model, batch_size)?;
    let embedded_coordinators: Vec<Vec<(String, Embedding)>> = coordinators
        .par_iter()
        .map(|x| {
            let coordinator_names: Vec<String> = x.par_iter().map(|x| x.name.clone()).collect();
            let coordinator_embeddings = passage_embed(coordinator_names, &model, batch_size).unwrap();
            x.par_iter()
                .zip(coordinator_embeddings)
                .map(|(x, y)| (x.email.clone(), y))
                .collect()
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

#[derive(Debug, Serialize, Deserialize)]
struct SearchQuery {
    query: String,
}


#[derive(Clone)]
struct AppState {
    db: Arc<PostgresDB>,
    model_ref: &'static TextEmbedding,
}

fn make_embedding_model() -> Result<TextEmbedding> {
    let model: TextEmbedding = TextEmbedding::try_new(InitOptions {
        //model_name: EmbeddingModel::MLE5Large,
        model_name: EmbeddingModel::ParaphraseMLMiniLML12V2,
        show_download_progress: true,
        ..Default::default()
    })?;
    Ok(model)
}


#[tokio::main]
async fn main() {
    let psql_pass = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD not set");
    let psql_user = env::var("POSTGRES_USER").expect("POSTGRES_USER not set");
    let psql_host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST not set");
    let psql_db = env::var("POSTGRES_DB").expect("POSTGRES_DB not set");

    let conn_string = format!("postgres://{}:{}@{}/{}", psql_user, psql_pass, psql_host, psql_db);

    let db = PostgresDB::new(&conn_string).await.expect("Failed to create database");
    migrate!("./migrations")
        .run(&db.pool)
        .await
        .expect("Failed to run migrations");



    let data_dir = env::var("DATA_DIR").expect("DATA_DIR not set");
    let new_json_dir = data_dir.to_owned() + "new_json/";
    let path = Path::new(&new_json_dir);


    let documents = read_jsons(path).unwrap();

    // get the modification times and title of all documents in the db
    let mut db_documents_map = std::collections::HashMap::new();
    let mod_times = db.get_document_mod_times().await.unwrap();

    for mod_time in mod_times.iter() {
        db_documents_map.insert(mod_time.0.clone(), mod_time.1);
    }

    // for each json document, check if it exists in the db, if it does, check if the json is newer than the db
    // if it is, update the db, if it isn't, do nothing
    let mut pending_documents = Vec::new();
    for document in documents.iter() {
        let path = format!("{}/{}.json", new_json_dir, document.info.id);
        let metadata = metadata(path).unwrap();
        let lastmodified = metadata.modified().unwrap();
        let db_lastmodified = db_documents_map.get(&document.info.id);
        match db_lastmodified {
            Some(db_lastmodified) => {
                if lastmodified.duration_since(std::time::UNIX_EPOCH)
                               .unwrap()
                               .as_secs() > *db_lastmodified as u64 {
                    pending_documents.push(document);
                }
            }
            None => {
                pending_documents.push(document);
            }
        }
    }

    println!("pending documents: {}", pending_documents.len());
    for document in pending_documents.iter() {
        db.upsert_document(document).await.unwrap();
    }

    let model = make_embedding_model().unwrap();

    // find all the courses that don't have an embedding
    let missing_embeddings = db.get_all_courses_wo_embeddings().await.unwrap();

    // grab all the missing documents
    // pub fn get_documents_by_ids(conn: &Connection, ids: &[String]) -> Result<Vec<Document>> {
    let missing_documents: Vec<Document> = db.get_documents_by_ids(&missing_embeddings).await.unwrap();
    println!("missing documents: {}", missing_documents.len());
    const BATCH_SIZE: usize = 32;
    // in batches of BATCH_SIZE, embed the documents and insert them into the db
    for batch in missing_documents.chunks(BATCH_SIZE) {
        let embedded_documents = embed_documents(batch.to_vec(), &model).unwrap();
        for embedded_document in embedded_documents.iter() {
            db.insert_embeddings(embedded_document).await.unwrap();
        }
        println!("inserted batch of {}", batch.len());
    }


    // Recreate the model since we just killed it
    let model = make_embedding_model().unwrap();

    let state = AppState {
        db: Arc::new(db),
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
    println!("listening on {}", port);
    axum::serve(listener, app).await.unwrap();
}

/*
fn document_similarity(
    query_embedding: &Embedding,
    embedded_document: &EmbeddedDocument,
) -> f32 {
    let content_similarity = dot_product(query_embedding, &embedded_document.content_embedding);

    // We weigh by 1.25 because we want the title to be more important than the content
    let title_similarity = dot_product(query_embedding, &embedded_document.title_embedding);
    let best_coordinator_similarity = embedded_document
        .coordinator_embeddings
        .iter()
        .map(|x| dot_product(query_embedding, &x.1))
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
*/
async fn ids_by_similarity(
    query: &str,
    db: &PostgresDB,
    model: &TextEmbedding,
) -> Vec<String> {
    let query_embedding = query_embed(query, &model).unwrap();

    let ids = db.get_most_relevant_course_ids(&query_embedding).await.unwrap();
    ids
}

async fn search(
    Query(query): Query<SearchQuery>,
    State(state): State<AppState>,
) -> Json<Vec<String>> {
    let query = query.query;
    let model = state.model_ref;
    let ids = ids_by_similarity(&query, &state.db, model);
    Json(ids.await)
}
