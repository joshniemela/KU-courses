use std::fs;
use tl;
use tl::VDom;
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

struct Course {
    id: String,
}

// this function returns a Result type
fn parse_course(html: &str) -> Result<Course, Box<dyn std::error::Error>> {
    let dom = tl::parse(html, tl::ParserOptions::default())?;
    let content = dom.get_element_by_id("content");

    // if there is no content element, we assume it is a new course
    if content.is_some() {
        return parse_old_course(&dom);
    }

    // 558 courses are new
    Err("Unknown course html format".into())
}

fn parse_old_course(dom: &VDom) -> Result<Course, Box<dyn std::error::Error>> {
    // find all div class="panel-body" elements and assert that there is only one
    let mut panel_bodies = dom.get_elements_by_class_name("panel-body");
    let parser = dom.parser();

    // there might be multiple panel-bodies, so we need to check each one
    // for the dl element (only the course info should have a dl element)
    for (i, panel_body) in panel_bodies.enumerate() {
        let mut dl_elements = panel_body
            .get(parser)
            .ok_or("Failed to get panel-body")?
            .as_tag()
            .ok_or("Failed to get panel-body as tag")?
            .query_selector(parser, "dl")
            .ok_or("Failed to get dl from panel-body")?;
        match dl_elements.next() {
            Some(handle) => {
                let node = handle
                    .get(parser)
                    .ok_or("Failed to get node")?
                    .as_tag()
                    .ok_or("Failed to get node as tag")?;
                // print the first 50 characters of the inner text
                println!("{}", node.inner_text(parser)[..51].to_string());
                println!("panel-body {}", i);
                return Err("Not implemented (parse candidate_body)".into());
            }
            None => continue,
        }
    }
    Err("No dl element found in the panel-body".into())
}

fn main() {
    // time how long it takes to run this
    let start = std::time::Instant::now();
    match get_course_filenames(PAGE_DIR) {
        Ok(filenames) => {
            let mut fails = 0;
            let mut passes = 0;
            // count the number of errors in a dictionary
            let mut errors: std::collections::HashMap<String, u32> =
                std::collections::HashMap::new();
            for filename in filenames {
                let path = format!("{}/{}", PAGE_DIR, filename);
                let html = fs::read_to_string(path).unwrap();
                let result = parse_course(&html);
                match result {
                    Ok(_) => passes += 1,
                    Err(err) => {
                        fails += 1;
                        let err_string = format!("{}", err);
                        let count = errors.entry(err_string).or_insert(0);
                        *count += 1;
                    }
                }
            }
            println!(
                "{} passes, {} fails\nparsed: {:.0}%",
                passes,
                fails,
                passes as f64 / (passes + fails) as f64 * 100.0
            );
            for (err, count) in errors.iter() {
                println!("{}: {}", err, count);
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
    println!("Time elapsed: {:.2?}", start.elapsed());
}
