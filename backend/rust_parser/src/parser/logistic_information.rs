use crate::parser;
use crate::parser::LogisticInformation;
use anyhow::{anyhow, bail, ensure, Context, Result};
use regex::Regex;
use tl::{NodeHandle, VDom};

// Convert two chars in a string to a u8
fn double_hex_to_u8(hex: &str) -> u8 {
    let mut chars = hex.chars();
    let first = chars.next().unwrap();
    let second = chars.next().unwrap();
    let first = first.to_digit(16).unwrap() as u8;
    let second = second.to_digit(16).unwrap() as u8;
    (first << 4) | second
}

fn deobfuscate_email(obfuscated_email: &str) -> Result<String> {
    let mut split = obfuscated_email.split("-");
    if split.clone().count() == 1 {
        return Ok(obfuscated_email.to_string());
    }
    let text = split.last().unwrap();

    let mut email = String::new();
    // Iterate through the split and move in steps of two
    // we offset the numbers by 0..25 since thats how they are obfuscated
    // if the regex matches an email we return it
    // else we continue incrementing the offset and hoping we find a match
    for i in 0..25 {
        for j in (0..text.clone().len()).step_by(2) {
            let hex = &text[j..j + 2];
            let u8 = double_hex_to_u8(hex) - i;
            email.push(u8 as char);
        }

        let regex = regex::Regex::new(r"(.+@.+\..+)").unwrap();
        if regex.is_match(&email) {
            return Ok(email);
        }
        email.clear();
    }
    bail!("Unable to deobfuscate email: {}", obfuscated_email)
}

pub fn parse_logistic_info(dom: &VDom) -> Result<LogisticInformation> {
    // Extract the information from the dom.
    let info: Vec<(String, Vec<String>)> = extract_h5_li_pairs(dom)?;

    let mut departments: Vec<parser::Department> = vec![];
    let mut coordinators: Vec<parser::Coordinator> = vec![];
    let mut faculty: Option<parser::Faculty> = None;

    for (h5, lis) in info {
        match h5.as_str() {
            "Kursusansvarlige" | "Course coordinators" => {
                for li in lis {
                    // the email should be removed from the name, it is enclosed in parenthesis
                    let mut split = li.split('(');
                    let name = split.next().unwrap().trim().to_string();
                    let obfuscated_email =
                        split.next().unwrap().split(')').next().unwrap().to_string();
                    let email = deobfuscate_email(&obfuscated_email)?;
                    coordinators.push(parser::Coordinator { name, email });
                }
            }
            "Udbydende fakultet" | "Contracting faculty" => {
                let faculty_str = lis.first().unwrap();
                match faculty_str.as_str() {
                    "Det Natur- og Biovidenskabelige Fakultet" | "Faculty of Science" => {
                        faculty = Some(parser::Faculty::Science)
                    }
                    _ => bail!("Unknown faculty: {} <EXPECTED>", faculty_str),
                }
            }
            "Udbydende institut" | "Contracting department" => {
                for li in lis {
                    departments.push(parser::Department::from_str(&li)?);
                }
            }

            &_ => {}
        }
    }

    Ok(parser::LogisticInformation {
        departments,
        faculty: faculty.unwrap(),
        coordinators,
    })
}

pub fn extract_h5_li_pairs(dom: &VDom) -> Result<Vec<(String, Vec<String>)>> {
    let parser = dom.parser();

    let raw_panel_bodies = dom.get_elements_by_class_name("panel-body");
    let panel_bodies =
        raw_panel_bodies.map(|panel_body| panel_body.get(parser).unwrap().as_tag().unwrap());

    let mut pairs: Vec<(String, Vec<String>)> = vec![];
    for (i, panel_body) in panel_bodies.enumerate() {
        let h5s = panel_body.query_selector(parser, "h5").unwrap();
        // if it contains h5s, we have found the right body
        if h5s.clone().count() > 0 {
            for h5 in h5s {
                let h5_text = h5.get(parser).unwrap().inner_text(parser).to_string();

                let inner_handle = h5.get_inner(); // This is the handle to the h5 tag

                // by magic we know that offsetting by 2 gives us the ul tag
                let ul_handle = NodeHandle::new(inner_handle + 2).get(parser).unwrap();

                // get the chldren inside of next_sibling
                let mut children = vec![];
                for child in ul_handle.as_tag().unwrap().children().top().iter() {
                    let child_text = child.get(parser).unwrap().inner_text(parser).to_string();
                    children.push(child_text);
                }
                pairs.push((h5_text, children));
            }
        }
    }
    Ok(pairs)
}
