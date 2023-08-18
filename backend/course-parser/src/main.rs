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

fn main() {
    match get_course_filenames(PAGE_DIR) {
        Ok(filenames) => {
            for filename in filenames {
                println!("{}", filename);
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
