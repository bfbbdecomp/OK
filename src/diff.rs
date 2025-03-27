use core::str;
use std::collections::HashMap;

use objdiff_core::bindings::report::{Report, ReportItem, ReportItemMetadata};

#[derive(Debug)]
pub struct DifferenceReport {
    pub functions: Vec<Difference>,
    pub sections: Vec<Difference>,
}

#[derive(Debug)]
pub struct Difference {
    pub unit: String,
    pub old: ReportItem,
    pub new: ReportItem,
}

pub type UnitItem = (String, ReportItem);

fn metadata_to_key(metadata: &Option<ReportItemMetadata>) -> String {
    serde_json::to_string(metadata).unwrap_or_else(|_| "null".to_string()) // Convert metadata to JSON
}

pub fn find_differences(previous: Report, current: Report) -> DifferenceReport {
    let (prev_fns, prev_secs) = relevant_items(&previous);
    let (curr_fns, curr_secs) = relevant_items(&current);

    let fn_diffs = get_item_differences(prev_fns, curr_fns);
    let sec_diffs = get_item_differences(prev_secs, curr_secs);

    DifferenceReport {
        functions: fn_diffs,
        sections: sec_diffs,
    }
}

pub fn get_item_differences(
    prev_items: Vec<UnitItem>,
    curr_items: Vec<UnitItem>,
) -> Vec<Difference> {
    // We need to hash everything but fuzzy match percent
    // and we also need to include the unit, name is not reliable.
    // the name 'GXSetTexCoordGen' appears across multiple units
    let mut old_map: HashMap<(String, String, u64, String), ReportItem> = HashMap::new();

    for (unit, item) in prev_items {
        old_map.insert(
            (
                unit.clone(),
                item.name.clone(),
                item.size,
                metadata_to_key(&item.metadata),
            ),
            item.clone(),
        );
    }

    let mut differences = Vec::new();

    for (unit, item) in curr_items {
        if let Some(old_item) = old_map.get(&(
            unit.clone(),
            item.name.clone(),
            item.size,
            metadata_to_key(&item.metadata),
        )) {
            if old_item.fuzzy_match_percent != item.fuzzy_match_percent {
                differences.push(Difference {
                    unit,
                    new: item,
                    old: old_item.clone(),
                });
            }
        }
    }

    differences
}

fn relevant_items(report: &Report) -> (Vec<UnitItem>, Vec<UnitItem>) {
    // We only care about Bob code
    // But we might want to report on everything if we're looking at a PR...
    let functions = report
        .units
        .iter()
        .filter(|u| u.name.contains("/SB/"))
        .flat_map(|u| u.functions.iter().map(|f| (u.name.clone(), f.clone())))
        .collect();

    let sections = report
        .units
        .iter()
        .filter(|u| u.name.contains("/SB/"))
        .flat_map(|u| u.sections.iter().map(|f| (u.name.clone(), f.clone())))
        .collect();

    (functions, sections)
}
