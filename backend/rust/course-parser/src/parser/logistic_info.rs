use crate::parser;
use tl::VDom;

pub fn parse(dom: &VDom) -> Result<parser::LogisticInformation, Box<dyn std::error::Error>> {
    // Extract the information from the dom.
    
    // Parse contracting department

    // Parse faculty
    
    // Parse coordinators
    unimplemented!()
}

fn extract_information() -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    unimplemented!()
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
