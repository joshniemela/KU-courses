use crate::parser;
use crate::parser::ExamInformation;
use anyhow::{anyhow, bail, ensure, Context, Result};

use tl::VDom;
pub fn parse_course_exams(dom: &VDom) -> Result<ExamInformation> {
    let parser = dom.parser();
    let exam_table = dom
        .get_element_by_id("course-exams1")
        .context("Unable to find exam table, this should never happen??? i think?")?
        .get(parser)
        .unwrap()
        .as_tag()
        .unwrap();

    let dds = exam_table
        .query_selector(parser, "dd")
        .context("Unable to find any dds, this should be impossible")?;
    let dts = exam_table
        .query_selector(parser, "dt")
        .context("Unable to find any dts, this should be impossible")?;

    ensure!(
        dds.count() == dts.count(),
        "Number of dds and dts in exam table does not match"
    );
    bail!("Not implemented yet");
}
