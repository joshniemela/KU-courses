use csv::WriterBuilder;
use std::env;

use anyhow::Result;
#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne};

use std::time;
pub mod parser;

//const DEFAULT_DATA_DIR: &str = "../../data";
//const HTMLS_DIR: &str = "../../data/pages";
//const TEST_DIR: &str = "./test_data";
//const TEST_HTMLS_DIR: &str = "./test_data/pages";
//const JSON_DIR: &str = "../../data/new_json";

// make a function that takes a path and returns the number of fails and the total number of courses
fn count_fails(htmls_dir: &str, json_dir: &str) -> (usize, usize) {
    let mut fails = 0;
    let mut passes = 0;
    let dir = std::fs::read_dir(htmls_dir).unwrap();
    let mut courses: Vec<parser::Course> = vec![];
    for entry in dir {
        let entry = entry.unwrap();
        // read the string from the file
        let html = std::fs::read_to_string(entry.path()).unwrap();
        // parse the string
        let course = parser::parse_course(&html);
        // if the error cause (this is an anyhow context) contains <EXPECTED>, then we ignore it and continue
        match course {
            Ok(c) => {
                // emit json to file
                let json = serde_json::to_string(&c).unwrap();
                let path = format!("{}/{}.json", json_dir, c.info.id);
                std::fs::write(path, json).unwrap();
                courses.push(c);
                passes += 1;
            }

            Err(e) => {
                println!("Error: {}", e);
                fails += 1;
            }
        }
    }
    write_courses_to_csv(courses).unwrap();
    (fails, passes)
}

fn reduce_workload_hours(workloads: Vec<parser::Workload>) -> f32 {
    let mut hours: Vec<f32> = vec![];
    for workload in workloads {
        hours.push(workload.hours);
    }
    hours.iter().sum()
}

fn write_courses_to_csv(courses: Vec<parser::Course>) -> Result<()> {
    let mut wtr = WriterBuilder::new().delimiter(b'\t').from_path("foo.tsv")?;
    wtr.write_record(&["title", "id", "ects", "hours", "faculty"])?;
    for course in courses {
        let hours = reduce_workload_hours(course.workloads);
        wtr.write_record(&[
            course.title,
            course.info.id,
            course.info.ects.to_string(),
            hours.to_string(),
            course.logistics.faculty.to_string(),
        ])?;
    }
    Ok(())
}

// take an in and out path as arguments
fn main() {
    let args: Vec<String> = env::args().collect();
    let timer = time::Instant::now();
    let html_dir = &args[1];
    let json_dir = &args[2];

    // print all files in the html directory
    let _dir = std::fs::read_dir(html_dir).unwrap();
    println!("fails and total: {:?}", count_fails(html_dir, json_dir));

    println!("Time elapsed: {:?}", timer.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    // check that there are files in PAGES_DIR
    #[test]
    fn test_pages_dir() {
        let dir = std::fs::read_dir(TEST_HTMLS_DIR).unwrap();
        assert!(dir.count() > 0);
    }

    #[test]
    fn test_LSLS10061U() {
        let html = std::fs::read_to_string(format!("{}/LSLS10061U.html", TEST_HTMLS_DIR)).unwrap();
        let course = parser::parse_course(&html);
        let expected_course = Course {
            title: "International Naturforvaltning".to_string(),
            info: parser::CourseInformation::new(
                "LSLS10061U".to_string(),
                7.5,
                vec![parser::Block::Two],
                vec![parser::Schedule::B],
                vec![parser::Language::Danish],
                parser::Duration::One,
                vec![parser::Degree::Bachelor],
                parser::Capacity(Some(70)),
            ),
        };
        pretty_assertions::assert_eq!(expected_course, course.unwrap());
    }

    // We need to ignore the duration if the course is known to be a summer course.
    #[test]
    fn test_NBIK15000U() {
        let html = std::fs::read_to_string(format!("{}/NBIK15000U.html", TEST_HTMLS_DIR)).unwrap();
        let course = parser::parse_course(&html);
        let expected_course = Course {
            title: "BAdvanced Plant Identification".to_string(),
            info: parser::CourseInformation::new(
                "NBIK15000U".to_string(),
                7.5,
                vec![parser::Block::Five],
                vec![parser::Schedule::B], // doesnt exist
                vec![parser::Language::English],
                parser::Duration::One,
                vec![parser::Degree::Master],
                parser::Capacity(Some(16)),
            ),
        };
        pretty_assertions::assert_eq!(expected_course, course.unwrap());
    }
}
