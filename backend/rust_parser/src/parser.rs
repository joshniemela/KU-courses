use anyhow::{anyhow, bail, ensure, Context, Result};
use tl::VDom;

use crate::parser::course_information::parse_course_info;
pub mod course_information;

use crate::parser::logistic_information::parse_logistic_info;
pub mod logistic_information;

#[derive(Debug, PartialEq)]
pub struct Course {
    pub title: String,
    pub info: CourseInformation,
}

#[derive(Debug)]
pub enum CourseLanguage {
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

enum Department {
    // Faculty of Science
    PlantAndEnvironmentalScience,
    Biology,
    ComputerScience,
    FoodAndResourceEconomics,
    FoodScience,
    GeosciencesAndNaturalResourceManagement,
    NutritionExerciseAndSports,
    Mathematics,
    ScienceEducation,
    PlantAndEnvironmentalSciences,
    Chemistry,
    NielsBohrInstitute,
    NaturalHistoryMuseumOfDenmark,
}
impl Department {
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Department of Computer Science" | "Datalogisk Institut" => {
                Ok(Department::ComputerScience)
            }
            "Institut for Idræt og Ernæring" | "Department of Nutrition, Exercise and Sports" => {
                Ok(Department::NutritionExerciseAndSports)
            }
            "Statens Naturhistoriske Museum" | "The Natural History Museum of Denmark" => {
                Ok(Department::NaturalHistoryMuseumOfDenmark)
            }
            "Institut for Plante- og Miljøvidenskab"
            | "Department of Plant and Environmental Sciences" => {
                Ok(Department::PlantAndEnvironmentalScience)
            }
            "Institut for Matematiske Fag" | "Department of Mathematical Sciences" => {
                Ok(Department::Mathematics)
            }
            "Niels Bohr Institutet" | "The Niels Bohr Institute" => {
                Ok(Department::NielsBohrInstitute)
            }
            "Institut for Geovidenskab og Naturforvaltning"
            | "Department of Geoscience and Natural Resource\nManagement" => {
                Ok(Department::GeosciencesAndNaturalResourceManagement)
            }
            "Institut for Naturfagenes Didaktik" | "Department of Science Education" => {
                Ok(Department::ScienceEducation)
            }
            "Institut for Fødevare- og Ressourceøkonomi"
            | "Department of Food and Resource Economics" => {
                Ok(Department::FoodAndResourceEconomics)
            }
            "Institut for Fødevarevidenskab" | "Department of Food Science" => {
                Ok(Department::FoodScience)
            }
            "Kemisk Institut" | "Department of Chemistry" => Ok(Department::Chemistry),
            "Biologisk Institut" | "Department of Biology" => Ok(Department::Biology),
            "Institut for Nordiske Studier og Sprogvidenskab" => {
                bail!("Nordic studies not supported <EXPECTED>")
            }
            _ => bail!("Unknown department: {}", s),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Faculty {
    Science,
}

#[derive(Debug, PartialEq)]
pub struct Coordinator {
    name: String,
    email: String,
}

pub struct LogisticInformation {
    departments: Vec<Department>,
    faculty: Faculty,
    coordinators: Vec<Coordinator>,
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
    One,
    Two,
    Three,
    Four,
    Summer,
    Other(String),
}

#[derive(Debug, PartialEq)]
pub enum Schedule {
    A,
    B,
    C,
    D,
    OutsideOfSchedule,
    Other(String),
}

#[derive(Debug, PartialEq)]
pub enum Language {
    Danish,
    English,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Duration {
    One,
    Two,
    DependsOnEcts,
    Custom(String),
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
    let logistic_info = parse_logistic_info(&dom).context(format!(
        "Unable to parse logistic information for course: {}",
        title
    ))?;

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
