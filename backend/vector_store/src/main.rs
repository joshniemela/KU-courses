use anyhow::Result;
use axum::extract::Query;
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use fastembed::{Embedding, EmbeddingModel, InitOptions, TextEmbedding};
use rayon::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::fs::metadata;
use std::path::Path;
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
    passages: Vec<String>,
    model: &TextEmbedding,
    batch_size: Option<usize>,
) -> Result<Vec<Embedding>> {
    // for each passage, add passage: to the front of it
    let passages: Vec<String> = passages
        .par_iter()
        .map(|x| format!("passage: {}", x))
        .collect();
    model.embed(passages, batch_size)
}

fn query_embed(query: &str, model: &TextEmbedding) -> Result<Embedding> {
    // add query: to the front of the query
    model
        .embed(vec![format!("query: {}", query)], None)
        .map(|x| x[0].clone())
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
    let embdded_titles = passage_embed(titles.clone(), model, batch_size)?;
    let embdded_descriptions = passage_embed(descriptions, model, batch_size)?;
    let embedded_coordinators: Vec<Vec<(String, Embedding)>> = coordinators
        .par_iter()
        .map(|x| {
            let coordinator_names: Vec<String> = x.par_iter().map(|x| x.name.clone()).collect();
            let coordinator_embeddings =
                passage_embed(coordinator_names, model, batch_size).unwrap();
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

fn embed_coordinator_names(
    emails: Vec<String>,
    names: Vec<String>,
    model: &TextEmbedding
) -> Result<Vec<(String, Embedding)>> {
    let embeddings = passage_embed(names, model, Some(32)).unwrap();
    let mut result = Vec::new();
    for i in 0..emails.len() {
        result.push((emails[i].clone(), embeddings[i].clone()));
    }
    Ok(result)
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
        model_name: EmbeddingModel::AllMiniLML12V2Q,
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

    let conn_string = format!(
        "postgres://{}:{}@{}/{}",
        psql_user, psql_pass, psql_host, psql_db
    );

    let db = PostgresDB::new(&conn_string)
        .await
        .expect("Failed to create database");
    migrate!("./migrations")
        .run(&db.pool)
        .await
        .expect("Failed to run migrations");

    let data_dir = env::var("DATA_DIR").expect("DATA_DIR not set");
    let new_json_dir = data_dir.to_owned() + "new_json/";
    let path = Path::new(&new_json_dir);

    let documents = read_jsons(path).unwrap();
    for document in documents.iter() {
        db.upsert_document(document).await.unwrap();
    }

    let model = make_embedding_model().unwrap();

    let missing_embeddings = db.get_outdated_embedding_course_ids().await.unwrap();

    let missing_documents: Vec<Document> =
        db.get_documents_by_ids(&missing_embeddings).await.unwrap();
    println!("missing documents: {}", missing_documents.len());
    const BATCH_SIZE: usize = 32;
    // in batches of BATCH_SIZE, embed the documents and insert them into the db
    for batch in missing_documents.chunks(BATCH_SIZE) {
        let embedded_documents = embed_documents(batch.to_vec(), &model).unwrap();
        for embedded_document in embedded_documents.iter() {
            let title_embedding = &embedded_document.title_embedding;
            let content_embedding = &embedded_document.content_embedding;

            db.insert_course_embeddings(
                &embedded_document.id,
                title_embedding,
                content_embedding,
            ).await.unwrap();
        }
        println!("inserted batch of {}", batch.len());
    }

    let missing_coordinator_names = db.get_missing_embedding_email_names().await.unwrap();

    for batch in missing_coordinator_names.chunks(BATCH_SIZE) {
        let name_embedding_pairs = embed_coordinator_names(
            batch.iter().map(|x| x.0.clone()).collect(),
            batch.iter().map(|x| x.1.clone()).collect(),
            &model
        ).unwrap();
        for (email, embedding) in name_embedding_pairs.iter() {
            db.insert_coordinator_embedding(email, embedding)
                .await
                .unwrap();
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
        .route("/name_similarities", get(name_search))
        .route("/title_similarities", get(title_search))
        .route("/content_similarities", get(content_search))
        .with_state(state);
    let addr = env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS must be set");
    let port = env::var("SERVER_PORT").expect("SERVER_PORT must be set");
    let listener = tokio::net::TcpListener::bind(&format!("{}:{}", addr, port))
        .await
        .unwrap();
    println!("listening on {}", port);
    axum::serve(listener, app).await.unwrap();
}

async fn ids_by_similarity(query: &str, db: &PostgresDB, model: &TextEmbedding) -> Vec<String> {
    let query_embedding = query_embed(query, model).unwrap();

    db.get_most_relevant_course_ids(&query_embedding)
        .await
        .unwrap()
}

async fn name_similarities(
    query: &str,
    db: &PostgresDB,
    model: &TextEmbedding,
) -> Vec<(String, f64)> {
    let query_embedding = query_embed(query, model).unwrap();

    db.get_name_similarities(&query_embedding).await.unwrap()
}

async fn name_search(
    Query(query): Query<SearchQuery>,
    State(state): State<AppState>,
) -> Json<Vec<(String, f64)>> {
    let query = query.query;
    let model = state.model_ref;
    let ids = name_similarities(&query, &state.db, model);
    Json(ids.await)
}

async fn title_similarities(
    query: &str,
    db: &PostgresDB,
    model: &TextEmbedding,
) -> Vec<(String, f64)> {
    let query_embedding = query_embed(query, model).unwrap();

    db.get_title_similarities(&query_embedding).await.unwrap()
}

async fn title_search(
    Query(query): Query<SearchQuery>,
    State(state): State<AppState>,
) -> Json<Vec<(String, f64)>> {
    let query = query.query;
    let model = state.model_ref;
    let ids = title_similarities(&query, &state.db, model);
    Json(ids.await)
}

async fn content_similarities(query: &str, db: &PostgresDB, model: &TextEmbedding) -> Vec<f64> {
    let query_embedding = query_embed(query, model).unwrap();

    db.get_content_similarities(&query_embedding).await.unwrap()
}
async fn content_search(
    Query(query): Query<SearchQuery>,
    State(state): State<AppState>,
) -> Json<Vec<f64>> {
    let query = query.query;
    let model = state.model_ref;
    let ids = content_similarities(&query, &state.db, model);
    Json(ids.await)
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
