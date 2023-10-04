use anyhow::{anyhow, ensure, Result};
use tl::VDom;

#[derive(Debug, PartialEq)]
pub struct Course {
    pub title: String,
}

pub fn parse_course(html: &str) -> Result<Course> {
    let dom = tl::parse(html, tl::ParserOptions::default())?;
    let _content = dom.get_element_by_id("content");
    let title = parse_title(&dom)?;
    Ok(Course { title })
}
fn parse_title(dom: &VDom) -> Result<String> {
    let title = dom
        .get_elements_by_class_name("courseTitle")
        .next()
        .ok_or_else(|| anyhow!("Unable to find course title"))
        .and_then(|elem| {
            elem.get(dom.parser())
                .ok_or_else(|| anyhow!("Unable to grab parser for the dom, this should not happen"))
                .map(|tag| tag.inner_text(dom.parser()))
        });

    let binding = title
        .unwrap_or_else(|_| "Error unwrapping html title".into())
        .replace(['\u{a0}', '\n'], " ");

    // Then split them
    let res: Vec<&str> = binding.split_whitespace().collect();

    // Return only the part of the title without the course code
    ensure!(
        res.len() >= 2,
        "Title does not conform to expected structure: <COURSECODE> <NAME>"
    );

    Ok(res[1..].join(" "))
}
