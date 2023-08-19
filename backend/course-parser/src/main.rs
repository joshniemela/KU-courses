use std::fs;
const DATA_DIR: &str = "../../data";
const PAGE_DIR: &str = "../../data/pages";

fn get_course_filenames(path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut filenames: Vec<String> = Vec::new();

    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if file_type.is_file() {
            let file_name = entry.file_name();
            filenames.push(file_name.to_string_lossy().to_string());
        }
    }

    Ok(filenames)
}

// this function returns a Result type
fn parser_fn(html: &str) -> Result<(), Box<dyn std::error::Error>> {
    Err("Not implemented".into())
}

fn try_parsing(html: &str, parser_fn: fn(&str) -> Result<(), Box<dyn std::error::Error>>) -> bool {
    match parser_fn(html) {
        Ok(_) => true,
        Err(err) => {
            eprintln!("Error: {}", err);
            false
        }
    }
}

fn main() {
    match get_course_filenames(PAGE_DIR) {
        Ok(filenames) => {
            let mut fails = 0;
            let mut passes = 0;
            for filename in filenames {
                let path = format!("{}/{}", PAGE_DIR, filename);
                let html = fs::read_to_string(path).unwrap();
                if try_parsing(&html, parser_fn) {
                    passes += 1;
                } else {
                    fails += 1;
                }
            }
            println!(
                "{} passes, {} fails\nparsed: {:.0}%",
                passes,
                fails,
                passes as f64 / (passes + fails) as f64 * 100.0
            );
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
