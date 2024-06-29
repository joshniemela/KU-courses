use super::{Coordinator, Course};

use anyhow::Result;
use async_stream::stream;
use fastembed::{Embedding, EmbeddingModel, InitOptions, TextEmbedding};
use futures_core::stream::Stream;
use rayon::prelude::*;

const BATCH_SIZE: usize = 32;

/// Embedding for a course
#[derive(Clone)]
pub struct CourseEmbedding {
    pub id: String,
    pub title: Embedding,
    pub content: Embedding,
}

/// Embedding for a coordinator
#[derive(Clone)]
pub struct CoordinatorEmbedding {
    pub email: String,
    pub name: Embedding,
}

/// Embedder for courses and coordinators
pub struct Embedder {
    pub model: TextEmbedding,
}

impl Embedder {
    pub fn new() -> Self {
        let model: TextEmbedding = TextEmbedding::try_new(InitOptions {
            model_name: EmbeddingModel::AllMiniLML12V2Q,
            show_download_progress: true,
            ..Default::default()
        })
        .expect("Failed to load model, please check your internet connection");
        Self { model }
    }

    /// Embeds a Vec<Course> into course embeddings
    /// This returns an asynchronous stream of CourseEmbedding
    pub fn embed_courses(
        &self,
        documents: Vec<Course>,
    ) -> impl Stream<Item = CourseEmbedding> + '_ {
        stream! {
            for batch in documents.chunks(BATCH_SIZE) {
                let embedded_courses = embed_course_batch(batch.to_vec(), &self.model).expect("Failed to embed courses, this should not happen");
                for embedded_course in embedded_courses.iter().cloned() {
                    yield embedded_course;
                }
                println!("Embedded batch of courses");
            }
        }
    }

    /// Embeds a Vec<Coordinator> into coordinator embeddings
    /// This returns an asynchronous stream of CoordinatorEmbedding
    pub fn embed_coordinators(
        &self,
        coordinators: Vec<Coordinator>,
    ) -> impl Stream<Item = CoordinatorEmbedding> + '_ {
        stream! {
            for batch in coordinators.chunks(BATCH_SIZE) {
                let model = &self.model;
                let embedded_coordinators = embed_coordinator_batch(
                    batch.to_vec(),
                    model
                ).expect("Failed to embed coordinators, this should not happen");
                for embedded_coordinator in embedded_coordinators.iter().cloned() {
                    yield embedded_coordinator;
                }
                println!("Embedded batch of coordinators");
            }
        }
    }

    // Embeds a query into an embedding
    // This returns an Embedding
    pub fn embed_query(&self, query: String) -> Embedding {
        query_embed(&query, &self.model).expect("Failed to embed query, this should not happen")
    }
}

/// Helper function to embed a batch of courses
/// This returns a Vec<CourseEmbedding>
fn embed_course_batch(courses: Vec<Course>, model: &TextEmbedding) -> Result<Vec<CourseEmbedding>> {
    let batch_size = Some(32);

    let embedded_titles = passage_embed(
        courses.par_iter().map(|x| x.title.clone()).collect(),
        model,
        batch_size,
    )?;

    let embedded_descriptions = passage_embed(
        courses.par_iter().map(|x| x.content.clone()).collect(),
        model,
        batch_size,
    )?;

    let embedded_courses: Vec<CourseEmbedding> = courses
        .iter()
        .cloned()
        .zip(embedded_titles.to_vec())
        .zip(embedded_descriptions.to_vec())
        .map(|((course, title), content)| CourseEmbedding {
            id: course.id,
            title,
            content,
        })
        .collect();

    Ok(embedded_courses)
}

/// Helper function to embed a batch of coordinators
/// This returns a Vec<CoordinatorEmbedding>
fn embed_coordinator_batch(
    coordinators: Vec<Coordinator>,
    model: &TextEmbedding,
) -> Result<Vec<CoordinatorEmbedding>> {
    let name_embeddings = passage_embed(
        coordinators.iter().map(|x| x.name.clone()).collect(),
        model,
        Some(32),
    )?;

    let coordinator_embeddings: Vec<CoordinatorEmbedding> = coordinators
        .iter()
        .cloned()
        .zip(name_embeddings.iter().cloned())
        .map(|(coordinator, name_embedding)| CoordinatorEmbedding {
            email: coordinator.email,
            name: name_embedding,
        })
        .collect();
    Ok(coordinator_embeddings)
}

/// Helper function to embed a list of passages
/// Passages are prepended with "passage: " before being embedded
/// This returns a Vec<Embedding>
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

/// Helper function to embed a query
/// The query is prepended with "query: " before being embedded
/// This returns an Embedding
fn query_embed(query: &str, model: &TextEmbedding) -> Result<Embedding> {
    // add query: to the front of the query
    model
        .embed(vec![format!("query: {}", query)], None)
        .map(|x| x[0].clone())
}
