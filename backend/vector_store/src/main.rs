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
use std::fs::{File, metadata};
use std::io::BufReader;
use std::path::Path;
use rayon::prelude::*;
use serde_json::json;

mod db;
use db::initialise_db;
use db::get_documents_by_ids;
use rusqlite::{params, Connection};
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
    coordinator_embeddings: Vec<Embedding>,
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

// db stuff
fn upsert_document(conn: &mut Connection, document: &Document) -> Result<()> {
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


fn insert_embeddings(conn: &mut Connection, embeddings: &EmbeddedDocument) -> Result<()> {
    conn.execute(
        "INSERT INTO embeddings (id, title_embedding, content_embedding, coordinator_embeddings) VALUES (?1, ?2, ?3, ?4)",
        params![embeddings.id,
                json![embeddings.title_embedding],
                json![embeddings.content_embedding],
                json![embeddings.coordinator_embeddings]],
    )?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let data_dir = env::var("DATA_DIR").expect("DATA_DIR not set");
    let new_json_dir = data_dir.to_owned() + "new_json/";
    let sql_path = data_dir.to_owned() + "documents.db";
    let path = Path::new(&new_json_dir);

    let mut conn = rusqlite::Connection::open(&sql_path).unwrap();
    initialise_db(&mut conn).unwrap();


    let documents = read_jsons(path).unwrap();

    // get the modification times and title of all documents in the db
    let mut stmt = conn.prepare("SELECT id, lastmodified FROM documents").unwrap();
    let mut db_documents_map = std::collections::HashMap::new();
    let db_documents_iter = stmt.query_map(params![], |row| {
        let id: String = row.get(0)?;
        let lastmodified: i64 = row.get(1)?;
        Ok((id, lastmodified))
    }).unwrap();

    for db_document in db_documents_iter {
        let db_document = db_document.unwrap();
        db_documents_map.insert(db_document.0, db_document.1);
    }
    drop(stmt);

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
        upsert_document(&mut conn, document).unwrap();
    }

    let model = make_embedding_model().unwrap();

    // find all the courses that don't have an embedding
    let mut stmt = conn.prepare("SELECT id FROM documents WHERE id NOT IN (SELECT id FROM embeddings)").unwrap();
    let mut missing_embeddings = Vec::new();
    let missing_embeddings_iter = stmt.query_map(params![], |row| {
        let id: String = row.get(0)?;
        Ok(id)
    }).unwrap();
    for missing_embedding in missing_embeddings_iter {
        let missing_embedding = missing_embedding.unwrap();
        missing_embeddings.push(missing_embedding);
    }
    drop(stmt);
    // grab all the missing documents
    // pub fn get_documents_by_ids(conn: &Connection, ids: &[String]) -> Result<Vec<Document>> {
    let missing_documents: Vec<Document> = get_documents_by_ids(&conn, &missing_embeddings).unwrap();
    println!("missing documents: {}", missing_documents.len());
    const BATCH_SIZE: usize = 32;
    // in batches of BATCH_SIZE, embed the documents and insert them into the db
    for batch in missing_documents.chunks(BATCH_SIZE) {
        let embedded_documents = embed_documents(batch.to_vec(), &model).unwrap();
        for embedded_document in embedded_documents.iter() {
            insert_embeddings(&mut conn, embedded_document).unwrap();
        }
        println!("inserted batch of {}", batch.len());
    }


    println!("grabbing embedded documents");
    let mut stmt = conn.prepare("SELECT id, title_embedding, content_embedding, coordinator_embeddings FROM embeddings").unwrap();
    let mut embedded_documents = Vec::new();
    let embedded_documents_iter = stmt.query_map(params![], |row| {
        let id: String = row.get(0)?;
        let title_embedding: String = row.get(1)?;
        let content_embedding: String = row.get(2)?;
        let coordinator_embeddings: String = row.get(3)?;
        Ok((id, title_embedding, content_embedding, coordinator_embeddings))
    }).unwrap();
    for embedded_document in embedded_documents_iter {
        let embedded_document = embedded_document.unwrap();
        let title_embedding: Embedding = serde_json::from_str(&embedded_document.1).unwrap();
        let content_embedding: Embedding = serde_json::from_str(&embedded_document.2).unwrap();
        let coordinator_embeddings: Vec<Embedding> = serde_json::from_str(&embedded_document.3).unwrap();
        let embedded_document = EmbeddedDocument {
            id: embedded_document.0,
            title_embedding,
            content_embedding,
            coordinator_embeddings,
        };
        embedded_documents.push(embedded_document);
    }


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
    println!("listening on {}", port);
    axum::serve(listener, app).await.unwrap();
}

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
        .map(|x| dot_product(query_embedding, x))
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
                document_similarity(&query_embedding, x),
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
