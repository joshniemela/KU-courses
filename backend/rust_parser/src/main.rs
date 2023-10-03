use std::{env::args, error::Error, time};

use crate::parser::Course;

pub mod parser;

const DEFAULT_DATA_DIR: &str = "../../data";

const HTMLS_DIR: &str = "../../data/pages";

const TEST_DIR: &str = "./test_data";

const TEST_HTMLS_DIR: &str = "./test_data/pages";

fn main() {
    let timer = time::Instant::now();

    // print all files in the html directory
    let dir = std::fs::read_dir(HTMLS_DIR).unwrap();
    for entry in dir {
        let entry = entry.unwrap();
        println!("{:?}", entry.path());
    }

    println!("Time elapsed: {:?}", timer.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    // check that there are files in PAGES_DIR
    #[test]
    fn test_pages_dir() {
        let dir = std::fs::read_dir(PAGES_DIR).unwrap();
        assert!(dir.count() > 0);
    }
}
