// File for the course info side-table
use crate::parser::{Capacity, CourseInformation, Duration};
use anyhow::{anyhow, bail, ensure, Context, Result};
use tl::VDom;

pub fn parse_course_info(dom: &VDom) -> Result<CourseInformation> {
    bail!("Not implemented yet");
    return Ok(CourseInformation {
        id: String::from(""),
        ects: 0.0,
        block: vec![],
        schedule: vec![],
        language: vec![],
        duration: Duration::Custom,
        degree: vec![],
        capacity: Capacity(None),
    });
}
