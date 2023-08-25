// File for the course info side-table
use crate::parser;
use tl::VDom;


/// Function that parsed the couse info section
///
/// # Parameters
/// * `dom: &VDom` - Reference to the DOM containing the course we want to parse
///
/// # Returns
/// A `CourseInformation` struct if succesful
///
/// # Errors
/// This can return a variety of errors, either those resulting from the tl crate,
/// or errors relating to unexpected failures in our business logic.
pub fn parse(dom: &VDom) -> Result<parser::CourseInformation, Box<dyn std::error::Error>> {
    // find all div class="panel-body" elements and assert that there is only one
    let panel_bodies = dom.get_elements_by_class_name("panel-body");
    let parser = dom.parser();

    // there might be multiple panel-bodies, so we need to check each one
    // for the dl element (only the course info should have a dl element)
    for (_i, panel_body) in panel_bodies.enumerate() {
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
                // parse DL
                let course_infos = parse_dl(node, parser)?;
                println!("{course_infos:?}");
                // parse the course information
                let coerced_course_info = coerce_course_info(course_infos, dom);
                return coerced_course_info;
            }
            None => continue,
        }
    }
    Err("No dl element found in the panel-body".into())
}

// return a list of tuples of (key, value)
fn parse_dl(
    dl_tag: &tl::HTMLTag,
    parser: &tl::Parser,
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let mut result: Vec<(String, String)> = Vec::new();
    let children = dl_tag.children();
    // for even numbers, we expect a dt element, odd numbers we expect a dd element
    // make a pair of precisely two strings
    let mut pair: Vec<String> = Vec::with_capacity(2);
    for (_i, child) in children.top().iter().enumerate() {
        let node = child
            .get(parser)
            .ok_or("Failed to get node whilst parsing dl")?;
        match node.as_tag() {
            Some(tag) => {
                if tag.name() == "dt" {
                    pair.push(node.inner_text(parser).to_string());
                } else if tag.name() == "dd" {
                    pair.push(node.inner_text(parser).to_string());
                    if pair.len() == 2 {
                        result.push((pair[0].clone(), pair[1].clone()));
                        pair.clear();
                    }
                } else {
                    return Err("Expected dt or dd element".into());
                }
            }
            None => continue,
        }
    }
    // if the pair is not empty then we have had an odd number of elements and should error
    if !pair.is_empty() {
        return Err("Odd number of elements in dl".into());
    }
    Ok(result)
}

fn parse_language(language: &str) -> Result<Vec<parser::Language>, Box<dyn std::error::Error>> {
    // println!("parser::Language information passed in: {language}");

    let mut languages: Vec<parser::Language> = Vec::new();

    if language.contains("Danish") | language.contains("Dansk") {
        languages.push(parser::Language::Danish);
    }

    if language.contains("English") | language.contains("Engelsk") {
        languages.push(parser::Language::English);
    }

    if languages.is_empty() {
        Err("Unable to parse languages".into())
    } else {
        Ok(languages)
    }
}

fn parse_ects(ects: &str, dom: &VDom) -> Result<f32, Box<dyn std::error::Error>> {
    // println!("Ects info: {ects}"); // Fixed formatting string

    // Extract numeric characters, '.' and ',' from the input string
    let ects_info = ects
        .chars()
        .take_while(|c| c.is_numeric() || *c == '.' || *c == ',')
        .collect::<String>();

    // Replace ',' with '.' to ensure correct parsing
    let ects_info = ects_info.replace(',', ".");

    // Parse the string to a f32
    let ects_value = ects_info.parse::<f32>().unwrap_or_else( |_| {
        // If we are unable to parse the ects values, it likely means that the field,
        // is instead saying something like "see description". Therefore we perform a full 
        // text search through the DOM as a last resort to see wether we can parse it.
        // We return -1.0 if we are unable to find it.
        let binding = dom.outer_html();
        let occurences: Vec<_> = binding.match_indices("ECTS").collect();
        println!("Despite being initially unable to parse float, found occurences of ECTS on indices:");
        for x in &occurences {
            println!("Index: {}", x.0);
        }

        // Next, if we're able to find some occurences of ECTS, we look at the potential numbers
        // preceeding the index, and extract the floats.
        let mut ects_values: Vec<f32> = Vec::new();
        for x in &occurences {
            let area = &binding[x.0-5..x.0+5];
            println!("Window index from: {} to {}", x.0-5, x.0);
            println!("Looking at window: {area}");
            let ects_val = area
                .chars()
                .filter(|c| c.is_numeric() || *c == '.' || *c == ',')
                .collect::<String>();

            let parsed_ects_val = ects_val.replace(',', ".");

            println!("Parsed ects value: {parsed_ects_val}");
            parsed_ects_val.parse::<f32>().map_or_else(|_| {
                println!("Unable to parse ects value in window, may just be because of a mention of ECTS without any number");
                }, |res| {
                    println!("Found ects value: {res} ECTS");
                    ects_values.push(res);
                });
        }
        let sum = ects_values.iter().sum();
        sum
    });
    if ects_value > 0.0 {
        Ok(ects_value)
    } else {
        Err("Unable to parse ECTS value!".into())
    }
}

#[allow(dead_code)]
fn parse_degree(degree: &str) -> Result<Vec<parser::Degree>, Box<dyn std::error::Error>> {
    // println!("parser::Degree information: {degree}");
    const WINDOW_LENGTH: usize = 4;
    let mut result: Vec<parser::Degree> = Vec::new();
    // loop through a 4 character sliding window and deal with the fact that they might not be alphabetic
    for i in 0..degree.len() - WINDOW_LENGTH - 1 {
        let sliding_window = &degree.to_lowercase()[i..i + WINDOW_LENGTH];
        match sliding_window {
            "bach" => result.push(parser::Degree::Bachelor),
            "mast" | "kand" => result.push(parser::Degree::Master),
            "ph.d" => result.push(parser::Degree::Phd),
            _ => continue,
        }
    }

    // Sort and deduplicate
    result.sort();
    result.dedup();
    if result.is_empty() {
        return Err("No degree found".into());
    }
    Ok(result)
}

fn parse_capacity(capacity: &str) -> parser::Capacity {
    // println!("parser::Capacity information passed in: {capacity}");

    // find the first number and parse it
    parser::Capacity(
        capacity
            .chars()
            .take_while(|c| c.is_numeric())
            .collect::<String>()
            .parse::<u32>()
            .ok(),
    )
}

fn parse_schedule(schedule: &str) -> Result<Vec<parser::Schedule>, Box<dyn std::error::Error>> {
    // println!("Schedule info passed in: {schedule}");
    let mut schedule_vec: Vec<parser::Schedule> = Vec::new();

    if schedule.contains('A') {
        schedule_vec.push(parser::Schedule::A);
    }
    if schedule.contains('B') {
        schedule_vec.push(parser::Schedule::B);
    }
    if schedule.contains('C') {
        schedule_vec.push(parser::Schedule::C);
    }
    if schedule.contains('D') {
        schedule_vec.push(parser::Schedule::D);
    }

    if schedule_vec.is_empty() {
        // Handle additional checks here
        println!("Schedule vector is empty!");
        println!("Schedule: {}", schedule.to_lowercase());
        if schedule.to_lowercase().contains("mon") {
            schedule_vec.push(parser::Schedule::B);
            schedule_vec.push(parser::Schedule::C);
        }
        if schedule.to_lowercase().contains("tue") {
            schedule_vec.push(parser::Schedule::A);
            schedule_vec.push(parser::Schedule::B);
        }
        if schedule.to_lowercase().contains("wed") {
            schedule_vec.push(parser::Schedule::C);
        }
        if schedule.to_lowercase().contains("thur") {
            schedule_vec.push(parser::Schedule::A);
        }
        if schedule.to_lowercase().contains("fri") {
            schedule_vec.push(parser::Schedule::B);
        }

        if schedule_vec.is_empty() {
            Err("Unknown schedule".into())
        } else {
            Ok(schedule_vec)
        }
    } else {
        Ok(schedule_vec)
    }
}

fn parse_block(input: &str, duration: &parser::Duration) -> Result<Vec<parser::Block>, Box<dyn std::error::Error>> {
    // println!("Block info: {input}");
    let mut blocks: Vec<parser::Block> = Vec::new();

    match duration {
        parser::Duration::One => {
            for c in input.chars() {
                match c {
                    '1' => blocks.push(parser::Block::One),
                    '2' => blocks.push(parser::Block::Two),
                    '3' => blocks.push(parser::Block::Three),
                    '4' => blocks.push(parser::Block::Four),
                    '5' => blocks.push(parser::Block::Five),
                    _ => (),
                }
            }
        },
        parser::Duration::Two => {
            // If they specify a duration of two. we first try to extract the blocks as before,
            // but if that fails, we try to search for "spring" etc.
            for c in input.chars() {
                match c {
                    '1' => blocks.push(parser::Block::One),
                    '2' => blocks.push(parser::Block::Two),
                    '3' => blocks.push(parser::Block::Three),
                    '4' => blocks.push(parser::Block::Four),
                    '5' => blocks.push(parser::Block::Five),
                    _ => (),
                }
            }
            if blocks.is_empty() {
                if input.contains("Spring") {
                    blocks.push(parser::Block::One);
                    blocks.push(parser::Block::Two);
                }
            }
        }
    }

    if blocks.is_empty() {
        Err("Unknown block".into())
    } else {
        Ok(blocks)
    }
}

fn parse_duration(duration: &str) -> Result<parser::Duration, Box<dyn std::error::Error>> {
    println!("Duration info: {duration}");
    // either 1 blo(c)k, 2 blo(c)ks or 1 semester
    // grab the first 3 chars
    match duration {
        x if duration.contains("blo") => {
            match x {
                _ if x.contains("1") => Ok(parser::Duration::One),
                _ if x.contains("2") => Ok(parser::Duration::Two),
                _ => Err("Unknown duration".into())
            }
        },
        _ if duration.contains("sem") => Ok(parser::Duration::Two),
        _ => Err("Unknown duration".into())
    }
}

fn coerce_course_info(
    course_info: Vec<(String, String)>,
    dom: &VDom
) -> Result<parser::CourseInformation, Box<dyn std::error::Error>> {
    // dbg!(&course_info);
    let mut id: Option<String> = None;
    let mut ects: Option<f32> = None;
    let mut block: Option<Vec<parser::Block>> = None;
    let mut schedule: Option<Vec<parser::Schedule>> = None;
    let mut language: Option<Vec<parser::Language>> = None;
    let mut duration: Option<parser::Duration> = None;
    let mut degree: Option<Vec<parser::Degree>> = None;
    let mut capacity: parser::Capacity = parser::Capacity(None);


    for (key, value) in &course_info {
        match key.as_str() {
            "Language" | "Sprog" => language = Some(parse_language(&value)?),
            "Course code" | "Kursuskode" => id = Some(value.clone()), // "Kursuskode" is the danish version of "Course code
            "Point" | "Credit" => ects = Some(parse_ects(&value, dom)?), // "Point" is the danish version of "Credit"
            "Level" | "Niveau" => degree = Some(parse_degree(&value)?),
            "Duration" | "Varighed" => duration = Some(parse_duration(&value)?),
            "Schedule" | "Skemagruppe" => schedule = Some(parse_schedule(&value)?),
            "Course capacity" | "Kursuskapacitet" => capacity = parse_capacity(&value),
            _ => continue
        }
    }

    // print every error with the contents of the course_info
    let id = id.ok_or_else(|| "Failed to get id")?;
    let ects = ects.ok_or_else(|| "Failed to get ECTS")?;
    let schedule = schedule.ok_or_else(|| "Failed to get schedule")?;
    let language = language.ok_or_else(|| "Failed to get language")?;
    let duration = duration.ok_or_else(|| "Failed to get duration")?;
    let degree = degree.ok_or_else(|| "Failed to get degree")?;
    

    for (key, value) in &course_info {
        // Since blocks might need information on the duration, we parse block afterwards
        match key.as_str() {
            "Placement" | "Placering" => block = Some(parse_block(value, &duration)?),
            _ => continue,
        }
    }
    let block = block.ok_or("Failed to get block")?;

    Ok(parser::CourseInformation {
        id,
        ects,
        block,
        schedule,
        language,
        duration,
        degree,
        capacity,
    })
}
