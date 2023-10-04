use crate::parser::Course;

#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne};

use std::time;
pub mod parser;

const DEFAULT_DATA_DIR: &str = "../../data";

const HTMLS_DIR: &str = "../../data/pages";

const TEST_DIR: &str = "./test_data";

const TEST_HTMLS_DIR: &str = "./test_data/pages";

// make a function that takes a path and returns the number of fails and the total number of courses
fn count_fails(htmls_dir: &str) -> (usize, usize) {
    let mut fails = 0;
    let mut passes = 0;
    let dir = std::fs::read_dir(htmls_dir).unwrap();
    for entry in dir {
        let entry = entry.unwrap();
        // read the string from the file
        let html = std::fs::read_to_string(entry.path()).unwrap();
        // parse the string
        let course = parser::parse_course(&html);
        // if the error cause (this is an anyhow context) contains <EXPECTED>, then we ignore it and continue
        match course {
            Ok(_) => passes += 1,
            Err(e) => {
                // if any of the causes contain <EXPECTED>, then we ignore it and continue
                if e.chain().any(|c| c.to_string().contains("<EXPECTED>")) {
                    continue;
                } else {
                    fails += 1;
                    println!("Error: {:?}\n\n", e);
                }
            }
        }
    }
    (fails, passes)
}

fn main() {
    let timer = time::Instant::now();

    // print all files in the html directory
    let _dir = std::fs::read_dir(HTMLS_DIR).unwrap();
    println!("fails and total: {:?}", count_fails(HTMLS_DIR));

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
