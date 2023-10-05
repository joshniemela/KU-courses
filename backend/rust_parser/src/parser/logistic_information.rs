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

    let mut pairs: Vec<(String, Vec<String>)> = vec![];
    for (i, panel_body) in panel_bodies.enumerate() {
        let h5s = panel_body.query_selector(parser, "h5").unwrap();
        // if it contains h5s, we have found the right body
        if h5s.clone().count() > 0 {
            for h5 in h5s {
                let h5_text = h5.get(parser).unwrap().inner_text(parser).to_string();

                let inner_handle = h5.get_inner(); // This is the handle to the h5 tag

                // by magic we know that offsetting by 2 gives us the ul tag
                let ul_handle = NodeHandle::new(inner_handle + 2).get(parser).unwrap();

                // get the chldren inside of next_sibling
                let mut children = vec![];
                for child in ul_handle.as_tag().unwrap().children().top().iter() {
                    let child_text = child.get(parser).unwrap().inner_text(parser).to_string();
                    children.push(child_text);
                }
                pairs.push((h5_text, children));
            }
        }
    }
    Ok(pairs)
}
