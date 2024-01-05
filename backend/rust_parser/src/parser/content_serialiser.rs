use crate::parser::Description;

use tl::VDom;

use anyhow::{Context, Result};

// grab some specific htmls and return the html
pub fn grab_htmls(dom: &VDom) -> Result<Description> {
    Ok(Description {
        content: "".to_string(),
        learning_outcome: "".to_string(),
        recommended_qualifications: None,
        summary: "".to_string(),
    })
}
