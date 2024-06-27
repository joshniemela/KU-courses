use axum::extract::Query;
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use std::env;
use std::path::Path;
use std::sync::Arc;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;
use sqlx::migrate;

mod db;
use db::PostgresDB;

mod populate;
use populate::upsert_documents_from_path;

mod embedding;
use embedding::Embedder;

#[derive(Clone)]
struct Course {
    id: String,
    title: String,
    content: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Coordinator {
    name: String,
    email: String,
}

#[derive(Clone)]
struct AppState {
    db: Arc<PostgresDB>,
    embedder: Arc<Embedder>,
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
    query: String,
}

/// Main function that starts the server
/// This function initializes the database, runs the migrations, and starts the server
/// A temporary functionality this server has is to populate the database with the documents
/// in the new_json directory, this will be removed in the future
/// The server has two endpoints:
///    - /health: returns "healthy" if the server is running
///    - /search: returns a list of course ids that most closely match the query
///
/// The server also has two background tasks that run every 6 hours:
///   - populate_coordinator_embeddings: updates the coordinator embeddings in the database
///   - populate_course_embeddings: updates the course embeddings in the database
///   These tasks use the embedder to generate the embeddings
#[tokio::main]
async fn main() {
    let conn_string = env::var("POSTGRES_URL").expect("POSTGRES_URL not set, it should be in the format postgres://user:password@host/db");

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
    upsert_documents_from_path(&db, path).await.unwrap();

    let state = AppState {
        db: Arc::new(db),
        embedder: Arc::new(Embedder::new())
    };

    const SYNC_INTERVAL: u64 = 60 * 60 * 6;

    let coordinator_state = state.clone();
    tokio::spawn(async move {
        loop {
            populate_coordinator_embeddings(&coordinator_state.db, &coordinator_state.embedder).await;
            println!("done populating coordinator embeddings");
            tokio::time::sleep(tokio::time::Duration::from_secs(SYNC_INTERVAL)).await;
        }
    });

    let course_state = state.clone();
    tokio::spawn(async move {
        loop {
            populate_course_embeddings(&course_state.db, &course_state.embedder).await;
            println!("done populating course embeddings");
            tokio::time::sleep(tokio::time::Duration::from_secs(SYNC_INTERVAL)).await;
        }
    });

    let app = Router::new()
        .route("/health", get(|| async { "healthy" }))
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

/// Search endpoint that takes a query parameter and returns a list of the course ids that
/// most closely match the query
async fn search(
    Query(query): Query<SearchQuery>,
    State(state): State<AppState>,
) -> Json<Vec<String>> {
    let query_embedding = state.embedder.embed_query(query.query);
    let db = &state.db;
    let ids = db.get_most_relevant_course_ids(&query_embedding)
        .await
        .unwrap();
    Json(ids)
}

/// Upserts the coordinator embeddings into the database using the coordinator information
/// from the database and the embedder to generate the embeddings
async fn populate_coordinator_embeddings(
    db: &PostgresDB,
    embedder: &Embedder,
) {
    let missing_coordinators = db.get_missing_embedding_email_names().await.unwrap();

    println!("missing coordinators: {}", missing_coordinators.len());

    let embedding_stream = embedder.embed_coordinators(missing_coordinators.clone());
    pin_mut!(embedding_stream);

    while let Some(embedded_coordinator) = embedding_stream.next().await {
        db.insert_coordinator_embedding(
            embedded_coordinator
        ).await.unwrap();
    }
}

/// Upserts the course embeddings into the database using the course information
/// from the database and the embedder to generate the embeddings
async fn populate_course_embeddings(
    db: &PostgresDB,
    embedder: &Embedder,
) {
    let outdated_embeddings = db.get_outdated_embedding_course_ids().await.unwrap();

    let outdated_courses: Vec<Course> =
        db.get_courses_by_ids(&outdated_embeddings).await.unwrap();

    println!("missing documents: {}", outdated_courses.len());

    let embedding_stream = embedder.embed_courses(outdated_courses.clone());
    pin_mut!(embedding_stream);

    while let Some(embedded_document) = embedding_stream.next().await {

        db.insert_course_embedding(
            embedded_document
        ).await.unwrap();
    }
}
