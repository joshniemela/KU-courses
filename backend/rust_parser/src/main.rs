use std::time;

use crate::parser::Course;
pub mod parser;

const DEFAULT_DATA_DIR: &str = "../../data";

const HTMLS_DIR: &str = "../../data/pages";

const TEST_DIR: &str = "./test_data";

const TEST_HTMLS_DIR: &str = "./test_data/pages";

// make a function that takes a path and returns the number of fails and the total number of courses
fn count_fails(htmls_dir: &str) -> (usize, usize) {
    let mut fails = 0;
    let mut total = 0;
    let dir = std::fs::read_dir(htmls_dir).unwrap();
    for entry in dir {
        let entry = entry.unwrap();
        // read the string from the file
        let html = std::fs::read_to_string(entry.path()).unwrap();
        // parse the string
        let course = parser::parse_course(&html);
        // print the course title
        if let Err(e) = course {
            println!("Error: {:?}", e);
            fails += 1;
        }
        total += 1;
    }
    (fails, total)
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
        let expected = Course {
            title: "International Naturforvaltning".to_string(),
        };
        assert_eq!(course.unwrap(), expected);
    }
}
