use super::{Coordinator, Course};

use anyhow::Result;
use async_stream::stream;
use fastembed::{Embedding, EmbeddingModel, InitOptions, TextEmbedding};
use futures_core::stream::Stream;
use rayon::prelude::*;

const BATCH_SIZE: usize = 32;

pub struct Embedder {
    pub model: TextEmbedding,
}

pub struct CourseEmbedding {
    pub id: String,
    pub title: Embedding,
    pub content: Embedding,
}

pub struct CoordinatorEmbedding {
    pub email: String,
    pub name: Embedding,
}

impl Embedder {
    pub fn new() -> Self {
        let model: TextEmbedding = TextEmbedding::try_new(InitOptions {
            model_name: EmbeddingModel::AllMiniLML12V2Q,
            show_download_progress: true,
            ..Default::default()
        })
        // unwrap is safe here because we know the model exists
        .unwrap();
        Self { model }
    }

    pub fn embed_courses(
        &self,
        documents: Vec<Course>,
    ) -> impl Stream<Item = CourseEmbedding> + '_ {
        stream! {
            for batch in documents.chunks(BATCH_SIZE) {
                let model = &self.model;
                let embedded_documents = embed_course_batch(batch.to_vec(), model).unwrap();
                for embedded_document in embedded_documents.iter() {
                    let title_embedding = &embedded_document.title;
                    let content_embedding = &embedded_document.content;

                    yield CourseEmbedding {
                        id: embedded_document.id.clone(),
                        title: title_embedding.clone(),
                        content: content_embedding.clone(),
                    };
                }
                println!("Embedded batch of courses");
            }
        }
    }

    pub fn embed_coordinators(
        &self,
        coordinators: Vec<Coordinator>,
    ) -> impl Stream<Item = CoordinatorEmbedding> + '_ {
        stream! {
            for batch in coordinators.chunks(BATCH_SIZE) {
                let model = &self.model;
                let name_embedding_pairs = embed_coordinator_batch(
                    batch.iter().map(|x| x.email.clone()).collect(),
                    batch.iter().map(|x| x.name.clone()).collect(),
                    model
                ).unwrap();
                for (email, embedding) in name_embedding_pairs.iter() {
                    yield CoordinatorEmbedding {
                        email: email.clone(),
                        name: embedding.clone(),
                    };
                }
                println!("Embedded batch of coordinators");
            }
        }
    }

    pub fn embed_query(&self, query: String) -> Embedding {
        query_embed(&query, &self.model).unwrap()
    }
}

fn embed_course_batch(courses: Vec<Course>, model: &TextEmbedding) -> Result<Vec<CourseEmbedding>> {
    let ids: Vec<String> = courses.par_iter().map(|x| x.id.clone()).collect();
    let titles: Vec<String> = courses.par_iter().map(|x| x.title.clone()).collect();
    let descriptions: Vec<String> = courses.par_iter().map(|x| x.content.clone()).collect();
    let batch_size = Some(32);
    let embdded_titles = passage_embed(titles.clone(), model, batch_size)?;
    let embdded_descriptions = passage_embed(descriptions, model, batch_size)?;

    let mut embedded_courses: Vec<CourseEmbedding> = Vec::new();
    for i in 0..courses.len() {
        let embedded_document = CourseEmbedding {
            id: ids[i].clone(),
            title: embdded_titles[i].clone(),
            content: embdded_descriptions[i].clone(),
        };
        embedded_courses.push(embedded_document);
    }
    Ok(embedded_courses)
}

fn embed_coordinator_batch(
    emails: Vec<String>,
    names: Vec<String>,
    model: &TextEmbedding,
) -> Result<Vec<(String, Embedding)>> {
    let embeddings = passage_embed(names, model, Some(32)).unwrap();
    let mut result = Vec::new();
    for i in 0..emails.len() {
        result.push((emails[i].clone(), embeddings[i].clone()));
    }
    Ok(result)
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
