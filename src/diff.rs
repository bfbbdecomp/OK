use std::collections::HashMap;

use objdiff_core::bindings::report::{ReportItem, ReportItemMetadata, ReportUnit};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DifferenceReport {
    pub functions: Vec<ReportItemDifference>,
    pub sections: Vec<ReportItemDifference>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ReportItemDifference {
    pub unit_name: String,
    pub name: String,
    pub size: u64,
    pub old_fuzzy_match_percent: f32,
    pub new_fuzzy_match_percent: f32,
    pub demangled_name: Option<String>,
    pub virtual_address: Option<u64>,
}

pub type UnitItem = (String, ReportItem);

pub fn find_differences(previous: Vec<ReportUnit>, current: Vec<ReportUnit>) -> DifferenceReport {
    let (prev_fns, prev_secs) = extract_items(previous);
    let (curr_fns, curr_secs) = extract_items(current);

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
) -> Vec<ReportItemDifference> {
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
                let metadata = item.metadata.as_ref();
                differences.push(ReportItemDifference {
                    unit_name: unit,
                    name: item.name,
                    size: item.size,
                    old_fuzzy_match_percent: old_item.fuzzy_match_percent,
                    new_fuzzy_match_percent: item.fuzzy_match_percent,
                    demangled_name: metadata.and_then(|x| x.demangled_name.clone()),
                    virtual_address: metadata.and_then(|x| x.virtual_address),
                });
            }
        }
    }

    differences
}

// Need to hash the metadata this way because ReportItemMetadata doesn't derive Eq
fn metadata_to_key(metadata: &Option<ReportItemMetadata>) -> String {
    serde_json::to_string(metadata).unwrap_or_else(|_| "null".to_string()) // Convert metadata to JSON
}

fn extract_items(units: Vec<ReportUnit>) -> (Vec<UnitItem>, Vec<UnitItem>) {
    let functions = units
        .iter()
        .flat_map(|u| u.functions.iter().map(|f| (u.name.clone(), f.clone())))
        .collect();

    let sections = units
        .iter()
        .flat_map(|u| u.sections.iter().map(|f| (u.name.clone(), f.clone())))
        .collect();

    (functions, sections)
}
