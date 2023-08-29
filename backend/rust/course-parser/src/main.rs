use std::env::args;
use std::{time};
use std::error::Error;
use storage_manager::{self, LocalStorage, LocalStorageConfig, Storage};

use crate::parser::Course;

pub mod parser;

const DEFAULT_DATA_DIR: &str = "../../../data";

const TEST_DIR: &str = "./test_data";

type ParsingResults = (i32, i32);

fn main() -> Result<(), Box<dyn Error>> {
    let timer = time::Instant::now();

    println!("Starting course parser...");

    // Pass in the directory to parse as the first argument, if none is specified, use the default
    // Passing in TEST_DIR as the first argument will use the test data.
    let args: Vec<String> = args().collect();

    let directory = args.get(1).map(|directory| directory.as_str())
        .unwrap_or(DEFAULT_DATA_DIR);

    let root = match directory {
        "TEST_DIR" => TEST_DIR,
        s => s
    };

    // Get files to parse
    let conf = LocalStorageConfig { root: root.to_string() };

    let storage = LocalStorage::new(conf).map_err(|_| {
        format!("Could not create storage with root: {}", root)
    }
    )?;

    let filenames = storage.list("pages", &0).map_err(|e|
        format!("Could not list files in storage: {}", e)
    )?;

    println!("Found {} files to parse", filenames.len());

    // Parse files
    let (passes, fails) = parse_files(&storage, filenames);

    // Print out the results
    println!("\n############## Results ##############");
    println!(
        "{} Passes, {} Fails\nSuccessfully Parsed: {:.2}%",
        passes,
        fails,
        (f64::from(passes) / f64::from(passes + fails)) * 100.0
    );

    println!("Time elapsed: {:.2?}", timer.elapsed());

    Ok(())
}

fn parse_files(storage: &impl Storage, filenames: Vec<String>) -> ParsingResults {
    let mut fails = 0;
    let mut passes = 0;

    let file_count = filenames.len();

    for filename in filenames {
        let course = try_parse_file(filename.as_str(), storage);

        // Since this is calculated before passes/fails are incremented, we add 1 to the total
        let courses_parsed = fails + passes + 1;

        match course {
            Ok(_) => {
                passes += 1;
                println!("[{}/{}] Successfully Parsed file: {}", courses_parsed, file_count, filename)
            }
            Err(e) => {
                fails += 1;
                println!("[{}/{}] Error parsing file: {}... Reason: {}", courses_parsed, file_count, filename, e);
            }
        }
    }
    (passes, fails)
}

fn try_parse_file(filename: &str, storage: &impl Storage) -> Result<Course, Box<dyn Error>> {
    let contents = storage.read(filename).map_err(|e|
        format!("Could not read file: {}", e)
    )?;

    return parser::parse_course(&contents);
}
