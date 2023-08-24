use eyre::Result;
use tl::VDom;

pub mod course_info_parser;

///////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURE
///////////////////////////////////////////////////////////////////////////////
#[allow(dead_code)]
pub struct Course {
    info: CourseInformation,
}

#[allow(dead_code)]
#[derive(Debug)]
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

#[derive(Debug)]
enum Block {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

#[derive(Debug)]
enum Schedule {
    A,
    B,
    C,
    D,
}

#[derive(Debug)]
enum Language {
    Danish,
    English,
}

#[derive(Debug, Eq, PartialEq)]
enum Duration {
    One = 1,
    Two = 2,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
enum Degree {
    Phd,
    Bachelor,
    Master,
}

#[derive(Debug)]
struct Capacity(pub Option<u32>);

///////////////////////////////////////////////////////////////////////////////
// LOGIC
///////////////////////////////////////////////////////////////////////////////

/// Parses html file.
///
/// Main entrypoint, and the function that gets called in main.rs.
///
/// # Parameters
/// * `html: &str` - `&str` representation of the contents of an html file
///
/// # Errors
/// Bubbles up the error resulting from any of functions called internally.
pub fn parse_course(html: &str) -> Result<Course, Box<dyn std::error::Error>> {
    let dom = tl::parse(html, tl::ParserOptions::default())?;
    let content = dom.get_element_by_id("content");
    let title = parse_title(&dom)?;
    println!("title: {:?}", title);

    // if there is no content element, we assume it is a new course
    if content.is_some() {
        let parsed_course_info = course_info_parser::parse_course_info(&dom)?;
        println!("{:?}", &parsed_course_info);
        return Ok(Course {
            info: parsed_course_info,
        });
    }

    Err("Unknown course html format".into())
}

fn parse_title(dom: &VDom) -> Result<String, Box<dyn std::error::Error>> {
    let title = dom
        .get_elements_by_class_name("courseTitle")
        .next()
        .expect("All courses should contain a title in a class with the name courseTitle")
        .get(dom.parser())
        .expect("Failed to get title, this should not happen")
        .as_tag()
        .expect("Failed to get title as tag, this should not happen")
        .inner_text(dom.parser());
        
    // First replace special characters
    let binding = title
        .replace("\u{a0}", " ")
        .replace("\n", " ");
    
    // Then split them
    let res: Vec<&str> = binding
        .split(" ")
        .collect();
    
    // Return only the part of the title without the course code
    Ok(res[1..].join(" ").to_string())

}

