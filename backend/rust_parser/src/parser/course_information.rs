// File for the course info side-table
use crate::parser;
use crate::parser::CourseInformation;
use anyhow::{bail, ensure, Context, Result};
use tl::VDom;

pub fn parse_course_info(dom: &VDom) -> Result<CourseInformation> {
    let parser = dom.parser();
    let panel_bodies = dom.get_elements_by_class_name("panel-body");
    // there might be multiple panel-bodies, so we need to check each one
    // for the dl element (only the course info should have a dl element)
    for (_i, panel_body) in panel_bodies.enumerate() {
        let mut dl_elements = panel_body
            .get(parser)
            .context("Failed to get panel-body")?
            .as_tag()
            .context("Failed to get panel-body as tag")?
            .query_selector(parser, "dl")
            .context("Failed to get dl from panel-body")?;
        match dl_elements.next() {
            Some(handle) => {
                let node = handle
                    .get(parser)
                    .context("Failed to get node")?
                    .as_tag()
                    .context("Failed to get node as tag")?;
                // parse DL
                let course_infos = parse_dl(node, parser)?;
                //println!("{course_infos:?}");
                // parse the course information
                let coerced_course_info = coerce_course_info(&course_infos, dom);
                return coerced_course_info;
            }
            None => continue,
        }
    }
    bail!("No dl element found in the panel-body");
}

fn coerce_course_info(
    course_info: &[(String, String)],
    dom: &VDom,
) -> Result<parser::CourseInformation> {
    // dbg!(&course_info);
    let mut id: Option<String> = None;
    let mut ects: Option<f32> = None;
    let mut block: Option<Vec<parser::Block>> = None;
    let mut schedule: Option<Vec<parser::Schedule>> = None;
    let mut language: Option<Vec<parser::Language>> = None;
    let mut degree: Option<Vec<parser::Degree>> = None;
    let mut capacity: parser::Capacity = parser::Capacity(None);

    // check the entire course_info first, if we do not have these 5 lines, we will encounter a race condition
    // where we try to parse courses that arent from SCIENCE and therefore get nonsensical results
    let course_code = course_info
        .iter()
        .find(|(key, _value)| key == "Course code" || key == "Kursuskode")
        .context("Failed to find course code (SHOULD BE IMPOSSIBLE)")?;
    parse_code(&course_code.1)?;

    for (key, value) in course_info {
        match key.as_str() {
            "Course code" | "Kursuskode" => id = Some(parse_code(value)?), // "Kursuskode" is the danish version of "Course code
            "Language" | "Sprog" => language = Some(parse_language(value)?),
            "Point" | "Credit" => ects = Some(parse_ects(value, dom)?), // "Point" is the danish version of "Credit"
            "Level" | "Niveau" => degree = Some(parse_degree(value)?),
            "Course capacity" | "Kursuskapacitet" => capacity = parse_capacity(value),
            _ => continue,
        }
    }

    // print every error with the contents of the course_info
    let id = id.context("Failed to get id")?;
    let ects = ects.context("Failed to get ECTS")?;
    let language = language.context("Failed to get language")?;
    let degree = degree.context("Failed to get degree")?;

    Ok(parser::CourseInformation {
        id,
        ects,
        block: vec![],
        schedule: vec![],
        language,
        duration: parser::Duration::One,
        degree,
        capacity,
    })
}

// Luca's research (2023-08-28)
// Name                    | Count                     | Unique IDS
// faculty of humanities: 173                          | HIOK, HTOB, HØEB, HØEK, HTYK, HÆGB, HEGR, HPOK, HSPK, HFMB, HFMK, HJAB, HKAK, HNAB, HNAK, HMØK, HRVB, HKUK, HLVK, HMVK, HENÅ, HASB, HHIB, HJAÆ, HÆGK, HTEK, HENB, HFAK, HCCK, HSAX, HKIÆ, HDCB, HIAB, HFIK, HFIB, HFRK, HKOB, HAIK, HENK, HANK, HIAÆ, HMØB, HHIK, HKIB, HMGK, HMKK, HOLD
// det sundhedsvidenskabelige fakultet: 402            | STVA, SFAK, SMBA, SFAB, SPMM, SMOB, SFOA, ITSK, SVEK, SMTB, STEF, SMEB, SVEB, SMOA, SFEB, SITK, SCIA, SODB, SASB, SITB, SGBK, SNRM, SSUK, SPUM, SSUA, SASA, SODK, SBIB, SMEA, SFOK, SSPE, SFOB, SITA, SGBB
// faculty of social sciences: 203                     | APSK, AØKB, ASOA, AGDK, AANK, ASRK, ASOK, ASTK, AØKA, ASDK, APSB, AØKK, AANB, AANA, ASOB
// det samfundsvidenskabelige fakultet: 181            | AØKB, ASOB, ASTB, POFK, APSK, AANB, ASOK, POFB, AANK, APSB, AØKK, ASTK, AANA, AØKA, ASOA
// det teologiske fakultet: 39                         | TISK, TTEA, TTBA
// faculty of law: 55                                  | JJUA, JJUS, JJUB
// faculty of health and medical sciences: 313         | SFKK, SFOK, SGBB, SFAB, SFKB, SGBK, SGLK, SASA, SITK, SKBK, SLKK, SMIM, SMOK, SMTB, SHDM, SVEK, SDMM, SMPS, SMEA, SMRM, SMTK, SMOB, SBIA, SITB, SHUA, SPMM, SNEU, SCAM, SBIB, SMKK, SASK, SIIK, SLVK, SSUA, SBIK, SFAK, SMOA, SMPM, SBRI, ITSE, SPEC
// det natur- og biovidenskabelige fakultet: 463       | NIGB, NBIB, NGEA, LFKB, NMAA, LMAB, LNAK, NFOB, NIFB, NNDK, LHUB, LLEB, NIGK, NDAB, NKEB, NBIK, NDIA, NMAB, NVIR, LNAB, LBIB, NDAA, NDAK, NMAK, NIDA, LOJB, NNDM, LKEB, NIGM, NPLB, NKEA, NNEB, LSLS, NNEM, NFYK, LPLB, NIDK, NIDB, NBIA, NNDB, NFYB, NFYA, NIFK, NNEK, NNMB, NGEB
// det juridiske fakultet: 102                         | JKOM, JJUA, JCSK, JJUS, JJUB, JKRD
// det humanistiske fakultet: 788                      | HFRB, HKGK, HRTK, HLIB, HEEB, HBAÆ, HPÆB, HMKK, HRUÆ, HSAX, TEMP, HSPK, HMVB, HLIK, HMSK, HITÅ, HJAB, HOLK, HFMB, HÆGK, HPLÆ, HØEB, HAUB, HLAÅ, HARÅ, HFIK, HKOB, HFIÅ, HKIK, HTÆK, HAIK, HDVÅ, HFIB, HSPÅ, HFPÅ, HTOR, HMØK, HKGÅ, HHIÅ, HHIK, HKUÅ, HØEK, HLAK, HIEK, HEGR, HMØB, HIMK, HKGÆ, HTYÅ, HKUK, HGAK, HLVK, HINK, HITÆ, HKGB, HPÆK, HKIB, HITB, HFRK, HLVB, HDAÅ, HEEK, HTOB, HDNÅ, HSSB, HDAK, HTEB, HFMK, HKIÆ, HIEB, HSSK, HÆGB, NORS, HLAB, HRVK, HTEK, HKMK, HITK, HFAB, HFAK, HNAB, HMVK, HIVB, HIAK, HMØÆ, HKMB, HRVÅ, HSPB, HLAÆ, HRTB, HINB, HKKK, HOLB, HRVB, HFRÅ, HHIB, HKAB, HTYK, HIAB, HJAÆ, HASK, HENG, HKVK, HJAK, HGAB, HAUK, HIAÆ, HDAB, HKAK, HKOK, HKUB, HVKK, HIVK, HTYB, HKOÆ, HGAÆ
// faculty of science: 575                             | NKEK, NDAA, NFKA, NFYB, LOJK, NIFK, NBIB, NFOB, LPLK, NNEB, NKEB, NNMB, NNDK, NPLK, NPIP, NFYK, NBIA, LLEK, LTEK, NIGK, NNEK, NKEA, NDAB, LNAK, NMAB, NIFB, NGEK, NIGB, NMAA, NFYA, NMAK, NNMK, LFKK, NDAK, NFOK, NBIK, LBIK, NPLB
// faculty of theology: 30                             | TISK, TAFA, TTER, TTEA
// From above it is clear that the faculty of science always can be matched on with the following regex: (N|L).*
// If the faculty is not from SCIENCE we want to return an error
// NORS is a special case, because its a humanities course (norwegian)
fn parse_code(code: &str) -> Result<String> {
    Ok(code.to_string())
}

fn parse_duration(duration: &str) -> Result<parser::Duration> {
    Ok(parser::Duration::One)
}

fn parse_block(input: &str, duration: &parser::Duration) -> Result<Vec<parser::Block>> {
    Ok(vec![parser::Block::One])
}

fn parse_schedule(schedule: &str) -> Result<Vec<parser::Schedule>> {
    Ok(vec![parser::Schedule::A])
}

fn parse_capacity(capacity: &str) -> parser::Capacity {
    // println!("parser::Capacity information passed in: {capacity}");

    // find the first number and parse it
    parser::Capacity(
        capacity
            .chars()
            .take_while(|c| c.is_numeric())
            .collect::<String>()
            .parse::<u32>()
            .ok(),
    )
}

fn parse_degree(degree: &str) -> Result<Vec<parser::Degree>> {
    Ok(vec![])
}

fn parse_ects(ects: &str, dom: &VDom) -> Result<f32> {
    // println!("Ects info: {ects}"); // Fixed formatting string

    // Extract numeric characters, '.' and ',' from the input string
    let ects_info = ects
        .chars()
        .filter(|c| c.is_numeric() || *c == '.' || *c == ',')
        .collect::<String>();

    // Replace ',' with '.' to ensure correct parsing
    let ects_info = ects_info.replace(',', ".");

    // Parse the string to a f32
    let ects_value = ects_info.parse::<f32>().unwrap_or_else(|_| {
        // If we are unable to parse the ects values, it likely means that the field,
        // is instead saying something like "see description". Therefore we perform a full
        // text search through the DOM as a last resort to see wether we can parse it.

        let binding = dom.outer_html();
        let occurences: Vec<_> = binding.match_indices("ECTS").collect();

        // Extract the ECTS values from the occurences
        let mut ects_values: Vec<f32> = Vec::new();
        for x in &occurences {
            if let Some(window) = binding.get(x.0 - 4..x.0) {
                let instance: String = window
                    .chars()
                    .filter(|x| x.is_numeric() || *x == ',' || *x == '.')
                    .collect::<String>();

                if let Ok(parsed_instance) = instance.replace(',', ".").parse::<f32>() {
                    ects_values.push(parsed_instance);
                }
            }
        }

        // After collecting the ects values, we sum them together for the final value
        let sum = ects_values.iter().sum();
        sum
    });

    ensure!(ects_value >= 0.0, "Unable to parse ECTS value!");
    Ok(ects_value)
}

fn parse_language(language: &str) -> Result<Vec<parser::Language>> {
    Ok(vec![])
}

// return a list of tuples of (key, value)
fn parse_dl(dl_tag: &tl::HTMLTag, parser: &tl::Parser) -> Result<Vec<(String, String)>> {
    let mut result: Vec<(String, String)> = Vec::new();
    let children = dl_tag.children();
    // for even numbers, we expect a dt element, odd numbers we expect a dd element
    // make a pair of precisely two strings
    let mut pair: Vec<String> = Vec::with_capacity(2);
    for (_i, child) in children.top().iter().enumerate() {
        let node = child
            .get(parser)
            .context("Failed to get node whilst parsing dl")?;
        match node.as_tag() {
            Some(tag) => {
                if tag.name() == "dt" {
                    pair.push(node.inner_text(parser).to_string());
                } else if tag.name() == "dd" {
                    pair.push(node.inner_text(parser).to_string());
                    if pair.len() == 2 {
                        result.push((pair[0].clone(), pair[1].clone()));
                        pair.clear();
                    }
                } else {
                    bail!("Expected dt or dd element");
                }
            }
            None => continue,
        }
    }
    // if the pair is not empty then we have had an odd number of elements and should error
    ensure!(pair.is_empty(), "Odd number of elements in dl");
    Ok(result)
}
