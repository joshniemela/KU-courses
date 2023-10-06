use crate::parser;
use crate::parser::{Workload, WorkloadType};
use anyhow::{anyhow, bail, ensure, Context, Result};

use tl::{NodeHandle, VDom};

pub fn parse_workloads(dom: &VDom) -> Result<Vec<Workload>> {
    let parser = dom.parser();
    let workload_table = dom
        .get_element_by_id("course-load")
        .context("Unable to find workload table")?
        .get(parser)
        .unwrap()
        .as_tag()
        .unwrap();

    let mut lis = workload_table
        .query_selector(parser, "li")
        .context("Unable to find any workload information")?
        .map(|node| node.get(parser).unwrap().inner_text(parser))
        .skip(2)
        .peekable();

    let mut workloads = Vec::new();
    // take two at a time from lis
    // first is the workload type
    // second is the workload value
    //
    // make
    let mut pair: Vec<String> = Vec::new();
    while lis.peek().is_some() {
        if pair.len() == 2 {
            let workload = Workload {
                workload_type: WorkloadType::from_str(&pair[0])?,
                hours: pair[1]
                    .replace(",", ".")
                    .parse::<f32>()
                    .context(format!("Unable to parse workload hours: {}", pair[1]))?,
            };
            workloads.push(workload);
            pair.clear();
        }
        pair.push(lis.next().unwrap().to_string());
    }

    ensure!(
        !workloads.is_empty(),
        "Unable to parse workload information"
    );
    Ok(workloads)
}
