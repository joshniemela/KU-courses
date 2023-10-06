use crate::parser;
use crate::parser::Exam;
use anyhow::{anyhow, bail, ensure, Context, Result};

use tl::{NodeHandle, VDom};

pub fn parse_course_exams(dom: &VDom) -> Result<Vec<Exam>> {
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

    let mut exams = Vec::<Exam>::new();
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
                    exams.push(parse_text_to_exam(&text)?);
                }
            }
            _ => continue,
        }
    }
    Ok(exams)
}

fn parse_text_to_exam(text: &str) -> Result<Exam> {
    let split = text.split(", ").collect::<Vec<&str>>();
    let exam_minutes = if split.clone().len() == 1 {
        None
    } else {
        // the first chars in split[1] are a duration in numbers
        let number = split[1]
            .chars()
            // take while numeric or a dot
            .take_while(|c| c.is_numeric() || *c == '.')
            .collect::<String>()
            .parse::<f32>()
            // convert error to nothing
            .unwrap_or(0.0);
        let factor = match split[1] {
            _ if split[1].contains("hour") || split[1].contains("time") => 60,
            _ if split[1].contains("day") || split[1].contains("dag") => 60 * 24,
            _ => 1, // We assume minutes if nothing else is specified
        };
        Some((number * factor as f32) as u32)
    };

    match split[0] {
        "Skriftlig aflevering" | "Written assignment" => Ok(Exam::Assignment(exam_minutes)),
        "Skriftlig prøve" | "Written examination" => Ok(Exam::Written(exam_minutes)),
        "Oral examination" | "Mundtlig prøve" => Ok(Exam::Oral(exam_minutes)),
        "Portfolio" | "Other" | "Andet" => Ok(Exam::Other),
        "Continuous assessment" | "Løbende bedømmelse" => Ok(Exam::ContinuousAssessment),
        _ => bail!("Not implemented for exam type: {}", split[0]),
    }
}
