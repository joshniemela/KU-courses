use crate::parser;
use crate::parser::LogisticInformation;
use anyhow::{anyhow, bail, ensure, Context, Result};
use tl::{NodeHandle, VDom};

pub fn parse_logistic_info(dom: &VDom) -> Result<LogisticInformation> {
    // Extract the information from the dom.
    let info: Vec<(String, Vec<String>)> = extract_h5_li_pairs(dom)?;

    println!("Extracted info: {info:?}");

    // Parse contracting department

    // Parse faculty

    // Parse coordinators
    Ok(parser::LogisticInformation {
        departments: vec![parser::Department::ComputerScience],
        faculty: parser::Faculty::Science,
        coordinators: vec![parser::Coordinator {
            name: "Kristian Pedersen".to_string(),
            email: "bs@org.dk".to_string(),
        }],
    })
}

pub fn extract_h5_li_pairs(dom: &VDom) -> Result<Vec<(String, Vec<String>)>> {
    let parser = dom.parser();

    let raw_panel_bodies = dom.get_elements_by_class_name("panel-body");
    let panel_bodies =
        raw_panel_bodies.map(|panel_body| panel_body.get(parser).unwrap().as_tag().unwrap());

    for (i, panel_body) in panel_bodies.enumerate() {
        //println!("Panel body {i}: {panel_body:?}");
        // does it contain a h5?
        let h5s = panel_body.query_selector(parser, "h5").unwrap();
        // return yes if more than one h5 is found
        if h5s.clone().count() > 0 {
            // print the h5s
            println!("Found h5s:");
            for h5 in h5s {
                println!("{h5:?}");

                let h5_text = h5.get(parser).unwrap().inner_text(parser);
                let inner_handle = h5.get_inner(); // This is the handle to the h5 tag
                let next_sibling = NodeHandle::new(inner_handle + 2).get(parser).unwrap();

                println!("h5 text: {h5_text:?}");
                //println!("Next sibling: {next_sibling:?}");
                // get the chldren inside of next_sibling
                for child in next_sibling.as_tag().unwrap().children().top().iter() {
                    let child_text = child.get(parser).unwrap().inner_text(parser);
                    println!("Child text: {child_text:?}");
                }
            }
        }
    }
    println!("\n\n");

    Ok(vec![])
}
