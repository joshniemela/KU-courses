use eyre::Result;
use tl::VDom;

pub mod course_info;
pub mod logistic_info;

///////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURE
///////////////////////////////////////////////////////////////////////////////
#[allow(dead_code)]
pub struct Course {
    title: String,
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

#[allow(dead_code)]
#[derive(Debug)]
pub struct LogisticInformation {
    contracting_departments: Vec<Department>,
    contracting_faculty: Faculty,
    course_coordinators: Vec<Coordinator>
}

#[allow(dead_code)]
#[derive(Debug)]
enum Department {
    // Faculty of Science
    PlantAndEnvironmentalScience,
    Biology,
    ComputerScience,
    FoodAndResourceEconomics,
    FoodScience,
    GeosciencesAndNaturalResourceManagement,
    NutritionExerciseAndSports,
    MathematicalScience,
    ScienceEducation,
    PlantAndEnvironmentalSciences,
    Chemistry,
    TheNielsBohrInstitute,
    NaturalHistoryMuseumOfDenmark
}

#[allow(dead_code)]
#[derive(Debug)]
enum Faculty {
    Science
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Coordinator {
    name: String,
    email: String,
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
    Custom,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
enum Degree {
    Phd,
    Bachelor,
    Master,
    Propædeutik
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
    println!("#####################################");
    let dom = tl::parse(html, tl::ParserOptions::default())?;
    let content = dom.get_element_by_id("content");
    let title = parse_title(&dom)?;
    println!("title: {title:?}");


    // if there is no content element, we assume it is a new course
    if content.is_some() {
        let logistic_information = logistic_info::parse(&dom)?;
        println!("{logistic_information:?}");
        println!("");
        let parsed_course_info = course_info::parse(&dom)?;
        println!("{parsed_course_info:?}");
        println!("##################################### \n");
        return Ok(Course {
            title,
            info: parsed_course_info,
        });
    }

    Err("Unknown course html format".into())
}

fn parse_title(dom: &VDom) -> Result<String, Box<dyn std::error::Error>> {
    let title = dom
        .get_elements_by_class_name("courseTitle")
        .next()
        .ok_or_else::<Box<dyn std::error::Error>, _>(|| "Unable to grab course title from dom".into())
        .and_then(|elem| {
            elem.get(dom.parser())
                .ok_or_else(|| "Unable to grab parser for the dom, this should not happen".into())
                .map(|tag| {
                    tag.inner_text(dom.parser())
                })
        });
    
    let binding = title.unwrap_or_else(|_ | "Error unwrapping html title".into()).replace(['\u{a0}', '\n'], " ");
    
    // Then split them
    let res: Vec<&str> = binding
        .split_whitespace()
        .collect();
    
    // Return only the part of the title without the course code
    if res.len() >= 2 {
        Ok(res[1..].join(" "))
    } else {
        Err("Title does not conform to expected structure: <COURSECODE> <NAME>".into())
    }

}

