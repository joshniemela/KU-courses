use crate::parser::Exam;
use anyhow::{bail, ensure, Context, Result};

use tl::{NodeHandle, VDom};

pub fn parse_course_exams(dom: &VDom) -> Result<Vec<Exam>> {
    Ok(Vec::new())
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
        _ if exam_name.contains("skriftlig prøve") || exam_name.contains("written exam") => {
            Ok(Exam::Written(exam_minutes))
        }
        _ if exam_name.contains("mundtlig prøve") || exam_name.contains("oral exam") => {
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
