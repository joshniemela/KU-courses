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
                for j in exam_boundary.0..exam_boundary.1 {
                    let text = NodeHandle::new(j).get(parser).unwrap().inner_text(parser);
                    exams.push(parse_text_to_exam(&text)?);
                }
                ensure!(
                    exams.len() > 0,
                    format!(
                        "No exams found in exam table: {}",
                        dd.get(parser).unwrap().inner_text(parser)
                    )
                );
            }
            _ => continue,
        }
    }
    if exams.len() > 1 && exams[0] == exams[1] {
        exams.remove(0);
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
            // convert error to Nothing type as number is an option type
            .ok();

        let factor = match split[1] {
            _ if split[1].contains("min") => Some(1),
            _ if split[1].contains("hour") || split[1].contains("time") => Some(60),
            _ if split[1].contains("day") || split[1].contains("dag") => Some(60 * 24),
            _ => None,
        };
        match (number, factor) {
            (None, _) => None,
            (_, None) => None,
            (Some(number), Some(factor)) => Some((number * factor as f32) as u32),
        }
    };

    let exam_name = split[0].to_lowercase().to_string();
    match exam_name {
        _ if exam_name.contains("aflevering") || exam_name.contains("assignment") => {
            Ok(Exam::Assignment(exam_minutes))
        }
        _ if exam_name.contains("skriftlig prøve") || exam_name.contains("written examination") => {
            Ok(Exam::Written(exam_minutes))
        }
        _ if exam_name.contains("mundtlig prøve") || exam_name.contains("oral examination") => {
            Ok(Exam::Oral(exam_minutes))
        }
        _ if exam_name.contains("portfolio")
            || exam_name.contains("other")
            || exam_name.contains("andet") =>
        {
            Ok(Exam::Other)
        }
        _ if exam_name.contains("løbende bedømmelse")
            || exam_name.contains("continuous assessment") =>
        {
            Ok(Exam::ContinuousAssessment)
        }
        _ => bail!("Not implemented for exam type: {}", split[0]),
    }
}
