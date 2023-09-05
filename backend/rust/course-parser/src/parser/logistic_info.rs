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
        course_coordinators: vec!(parser::Coordinator {
            name: "Kristian Pedersen".to_string(),
            email: "bs@org.dk".to_string()
        })
    })
}


fn extract_information(dom: &VDom) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let parser = dom.parser();

    // Find all div class="panel-body" elements
    let panel_bodies = dom
        .get_elements_by_class_name("panel-body")
        .next()
        .unwrap()
        .get(parser)
        .unwrap()
        .as_tag()
        .unwrap();

    let mut dept_heading: Option<String> = None;
    let mut faculty_heading: Option<String> = None;
    let mut coordinators_heading: Option<Vec<String>> = None;

    let mut dept: Option<String> = None;
    let mut faculty: Option<String> = None;
    let mut coordinators: Option<Vec<String>> = None;

    // Iterate over h5's
    let h5 = panel_bodies.query_selector(parser, "h5").unwrap();
    h5.for_each(|h5_node| {
            let html_inner = h5_node.get(parser).unwrap().inner_text(parser);
            println!("{:?}", &html_inner);
            match &html_inner {
                x if x.contains("Department") => dept = Some(html_inner.to_string()),
                _ => ()
            }
    });

    // Iterate over the lists
    let mut lists: Vec<String> = Vec::new();
    let uls = panel_bodies.query_selector(parser, "ul").unwrap();
    uls.for_each(|ul| {
        let li = ul.get(parser).unwrap().as_tag().unwrap().inner_text(parser);
        println!("Content: {}", li);
        match &li.to_lowercase() {
            x if x.contains("department") => dept_heading = Some(li.to_string()),
            _ => ()
        }
    });

    let mut res: Vec<(String, String)> = Vec::new();

    if dept_heading.is_some() && dept.is_some() {
        res.push((dept_heading.unwrap(), dept.unwrap()));
    }


    Ok(res)
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
