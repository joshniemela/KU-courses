use anyhow::{anyhow, bail, ensure, Context, Result};
use serde::Serialize;
use tl::VDom;

use crate::parser::course_information::parse_course_info;
pub mod course_information;

use crate::parser::exam_information::parse_course_exams;
pub mod exam_information;

use crate::parser::logistic_information::parse_logistic_info;
pub mod logistic_information;

use crate::parser::workload_information::parse_workloads;
pub mod workload_information;

use crate::parser::content_serialiser::grab_htmls;
pub mod content_serialiser;

#[derive(Debug, PartialEq, Serialize)]
pub struct Course {
    pub title: String,
    pub info: CourseInformation,
    pub logistics: LogisticInformation,
    pub workloads: Vec<Workload>,
    pub exams: Vec<Exam>,
    pub description: Description,
}

#[derive(Debug)]
pub enum CourseLanguage {
    English,
    Danish,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct CourseInformation {
    pub id: String,
    ects: f32,
    block: Vec<Block>,
    schedule: Vec<Schedule>,
    language: Vec<Language>,
    duration: Duration,
    degree: Vec<Degree>,
    capacity: Capacity,
}

#[derive(Debug, PartialEq, Serialize)]
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
    VeterinaryAndAnimalSciences,
    BiomedicalSciences,
    PublicHealth,
    DrugDesignAndPharmacology,
    CellularAndMolecularMedicine,
}
impl Department {
    fn from_str(s: &str) -> Result<Self> {
        match s.replace("\n", " ").as_str() {
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
            | "Department of Geoscience and Natural Resource Management" => {
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
            "Department of Veterinary and Animal Sciences"
            | "Institut for Veterinær- og Husdyrvidenskab (IVH)" => {
                Ok(Department::VeterinaryAndAnimalSciences)
            }
            "Department of Biomedical Sciences" => Ok(Department::BiomedicalSciences),
            "Institut for Lægemiddeldesign og Farmakologi"
            | "Department of Drug Design and Pharmacology" => {
                Ok(Department::DrugDesignAndPharmacology)
            }
            "Department of Cellular and Molecular Medicine" => {
                Ok(Department::CellularAndMolecularMedicine)
            }
            "Department of Public Health" => Ok(Department::PublicHealth),
            "Institut for Nordiske Studier og Sprogvidenskab" => {
                bail!("Nordic studies not supported <EXPECTED>")
            }
            _ => bail!("Unknown department: {}", s),
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
enum Faculty {
    Science,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Coordinator {
    name: String,
    email: String,
}

#[derive(Debug, PartialEq, Serialize)]
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

#[derive(Debug, PartialEq, Serialize)]
pub enum Block {
    One,
    Two,
    Three,
    Four,
    Summer,
    Other(String),
}

#[derive(Debug, PartialEq, Serialize)]
pub enum Schedule {
    A,
    B,
    C,
    D,
    OutsideOfSchedule,
    Other(String),
}

#[derive(Debug, PartialEq, Serialize)]
pub enum Language {
    Danish,
    English,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub enum Duration {
    One,
    Two,
    DependsOnEcts,
    Custom(String),
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Serialize)]
pub enum Degree {
    Phd,
    Bachelor,
    Master,
    Propædeutik,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Capacity(pub Option<u32>);

#[derive(Debug, PartialEq, Serialize)]
pub enum Exam {
    Oral(Option<u32>),
    Written(Option<u32>),
    Assignment(Option<u32>),
    ContinuousAssessment,
    Other,
}

#[derive(Debug, PartialEq, Serialize)]
enum WorkloadType {
    Exam,
    ELearning,
    Laboratory,
    StudyGroup,
    TheoryExercises,
    FieldWork,
    Preparation,
    ExamPreparation,
    Excursions,
    Lectures,
    PracticalExercises,
    ProjectWork,
    Exercises,
    Guidance,
    ClassInstruction,
    PracticalTraining,
    Seminar,
}
impl WorkloadType {
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "Forelæsninger" | "Lectures" => Ok(WorkloadType::Lectures),
            "Forberedelse (anslået)" | "Preparation" => Ok(WorkloadType::Preparation),
            "Theory exercises" | "Teoretiske øvelser" => Ok(WorkloadType::TheoryExercises),
            "Project work" | "Projektarbejde" => Ok(WorkloadType::ProjectWork),
            "Øvelser" | "Exercises" => Ok(WorkloadType::Exercises),
            "Eksamen" | "Exam" => Ok(WorkloadType::Exam),
            "Eksamensforberedelse" | "Exam Preparation" => Ok(WorkloadType::ExamPreparation),
            "Holdundervisning" | "Class Instruction" => Ok(WorkloadType::ClassInstruction),
            "Practical exercises" | "Praktiske øvelser" => Ok(WorkloadType::PracticalExercises),
            "E-Learning" | "E-læring" => Ok(WorkloadType::ELearning),
            "Feltarbejde" | "Field Work" => Ok(WorkloadType::FieldWork),
            "Studiegrupper" | "Study Groups" => Ok(WorkloadType::StudyGroup),
            "Seminar" => Ok(WorkloadType::Seminar),
            "Ekskursioner" | "Excursions" => Ok(WorkloadType::Excursions),
            "Laboratorie" | "Laboratory" => Ok(WorkloadType::Laboratory),
            "Praktik" | "Practical Training" => Ok(WorkloadType::PracticalTraining),

            "Guidance" | "Vejledning" => Ok(WorkloadType::Guidance),
            _ => bail!("Unknown workload type: {}", s),
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Workload {
    workload_type: WorkloadType,
    hours: f32,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Description {
    pub content: String,
    pub learning_outcome: String,
    pub recommended_qualifications: Option<String>,
    pub summary: String,
}

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

    let exam_info = parse_course_exams(&dom).context(format!(
        "Unable to parse exam information for course: {}",
        title
    ))?;

    let workload_info = parse_workloads(&dom).context(format!(
        "Unable to parse workload information for course: {}",
        title
    ))?;

    let html_info = grab_htmls(&dom).context(format!(
        "Unable to grab html information for course: {}",
        title
    ))?;

    Ok(Course {
        title,
        info,
        logistics: logistic_info,
        exams: exam_info,
        workloads: workload_info,
        description: html_info,
    })
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
