use anyhow::{anyhow, bail, ensure, Context, Result};
use tl::VDom;

use crate::parser::course_information::parse_course_info;
pub mod course_information;

#[derive(Debug, PartialEq)]
pub struct Course {
    pub title: String,
    pub info: CourseInformation,
}

#[derive(Debug)]
enum CourseLanguage {
    English,
    Danish,
}

#[derive(Debug, PartialEq)]
pub struct CourseInformation {
    id: String,
    ects: f32,
    block: Vec<Block>,
    schedule: Vec<Schedule>,
    language: Vec<Language>,
    duration: Duration,
    degree: Vec<Degree>,
    capacity: Capacity,
}
impl CourseInformation {
    pub fn new(
        id: String,
        ects: f32,
        block: Vec<Block>,
        schedule: Vec<Schedule>,
        language: Vec<Language>,
        duration: Duration,
        degree: Vec<Degree>,
        capacity: Capacity,
    ) -> Self {
        Self {
            id,
            ects,
            block,
            schedule,
            language,
            duration,
            degree,
            capacity,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Block {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

#[derive(Debug, PartialEq)]
pub enum Schedule {
    A,
    B,
    C,
    D,
}

#[derive(Debug, PartialEq)]
pub enum Language {
    Danish,
    English,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Duration {
    One = 1,
    Two = 2,
    Custom,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Degree {
    Phd,
    Bachelor,
    Master,
    Propædeutik,
}

#[derive(Debug, PartialEq)]
pub struct Capacity(pub Option<u32>);

pub fn parse_course(html: &str) -> Result<Course> {
    let dom = tl::parse(html, tl::ParserOptions::default())?;
    let content = dom.get_element_by_id("content");
    let title = parse_title(&dom)?;

    ensure!(
        content.is_some(),
        "Unable to find content element, this should not happen"
    );
    let info = parse_course_info(&dom).context(format!("Unable to parse course: {}", title))?;

    Ok(Course { title, info })
}
fn parse_title(dom: &VDom) -> Result<String> {
    let title = dom
        .get_elements_by_class_name("courseTitle")
        .next()
        .context("Unable to find course title")
        .and_then(|elem| {
            elem.get(dom.parser())
                .context("Unable to grab parser for the dom, this should not happen")
                .map(|tag| tag.inner_text(dom.parser()))
        });

    let binding = title
        .unwrap_or_else(|_| "Error unwrapping html title".into())
        .replace(['\u{a0}', '\n'], " ");

    // Then split them
    let res: Vec<&str> = binding.split_whitespace().collect();

    // Return only the part of the title without the course code
    ensure!(
        res.len() >= 2,
        "Title does not conform to expected structure: <COURSECODE> <NAME>"
    );

    Ok(res[1..].join(" "))
}
// not implemented yet, just return an empty course info
