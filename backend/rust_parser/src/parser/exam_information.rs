use crate::parser;
use crate::parser::ExamInformation;
use anyhow::{anyhow, bail, ensure, Context, Result};

use tl::{NodeHandle, VDom};

pub fn parse_course_exams(dom: &VDom) -> Result<ExamInformation> {
    let parser = dom.parser();
    let exam_table = dom
        .get_element_by_id("course-exams1")
        .context("Unable to find exam table, this should never happen??? i think?")?
        .get(parser)
        .unwrap()
        .as_tag()
        .unwrap();

    let dts = exam_table
        .query_selector(parser, "dt")
        .context("Unable to find any dts, this should be impossible")?;
    let dds = exam_table
        .query_selector(parser, "dd")
        .context("Unable to find any dds, this should be impossible")?;

    ensure!(
        dds.clone().count() == dts.clone().count(),
        "Number of dds and dts in exam table does not match"
    );
    for (dt, dd) in dts.zip(dds) {
        let dt_text = dt.get(parser).unwrap().inner_text(parser).to_string();
        match dt_text.as_str() {
            "Type of assessment" | "Prøveform" => {
                let exam_boundary = dd
                    .get(parser)
                    .unwrap()
                    .children()
                    .unwrap()
                    .boundaries(parser)
                    .unwrap();
                for j in (exam_boundary.0 + 1..exam_boundary.1) {
                    let text = NodeHandle::new(j).get(parser).unwrap().inner_text(parser);
                    println!("exam number {}, text: {}", j, text);
                }
            }
            _ => continue,
        }
    }

    bail!("Not implemented yet");
}
