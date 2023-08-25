use crate::parser;
use tl::VDom;

pub fn parse(dom: &VDom) -> Result<parser::LogisticInformation, Box<dyn std::error::Error>> {
    // Extract the information from the dom.
    let info: Vec<(String, String)> = extract_information(dom)?;

    println!("Extracted info: {info:?}");
    
    // Parse contracting department

    // Parse faculty
    
    // Parse coordinators
    Ok(parser::LogisticInformation {
        contracting_departments: vec!(parser::Department::ComputerScience),
        contracting_faculty: parser::Faculty::Science,
        course_coordniators: vec!(parser::Coordinator {
            name: "Kristian Pedersen".to_string(),
            email: "bs@org.dk".to_string()
        })
    })
}

fn extract_information(dom: &VDom) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    // // find all div class="panel-body" elements and assert that there is only one
    // let panel_bodies = dom.get_elements_by_class_name("panel-body");
    // let parser = dom.parser();
    //
    // // there might be multiple panel-bodies, so we need to check each one
    // // for the dl element (only the course info should have a dl element)
    // for (_i, panel_body) in panel_bodies.enumerate() {
    //     let mut dl_elements = panel_body
    //         .get(parser)
    //         .ok_or("Failed to get panel-body")?
    //         .as_tag()
    //         .ok_or("Failed to get panel-body as tag")?
    //         .query_selector(parser, "dl")
    //         .ok_or("Failed to get dl from panel-body")?;
    //     match dl_elements.next() {
    //         Some(handle) => {
    //             let node = handle
    //                 .get(parser)
    //                 .ok_or("Failed to get node")?
    //                 .as_tag()
    //                 .ok_or("Failed to get node as tag")?;
    //             // parse DL
    //             let course_infos = parse_dl(node, parser)?;
    //             //println!("{course_infos:?}");
    //             // parse the course information
    //             let coerced_course_info = coerce_course_info(&course_infos, dom);
    //             return coerced_course_info;
    //         }
    //         None => continue,
    //     }
    // }
    // Err("No dl element found in the panel-body".into())

    Ok(vec!((String::from("Test"), String::from("Value"))))
}

fn parse_contracting_departments(input: &str) -> Result<Vec<parser::Department>, Box<dyn std::error::Error>> {
    unimplemented!()
}

fn parse_contracting_faculty(input: &str) -> Result<Vec<parser::Faculty>, Box<dyn std::error::Error>> {
    unimplemented!()
}

fn parse_coordinators(input:&str) -> Result<Vec<parser::Coordinator>, Box<dyn std::error::Error>> {
    unimplemented!()
}
