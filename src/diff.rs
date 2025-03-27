use std::collections::HashMap;

use objdiff_core::bindings::report::{Report, ReportItem, ReportItemMetadata};

#[derive(Debug)]
pub struct Difference {
    pub old_item: ReportItem,
    pub new_item: ReportItem,
}

fn metadata_to_key(metadata: &Option<ReportItemMetadata>) -> String {
    serde_json::to_string(metadata).unwrap_or_else(|_| "null".to_string()) // Convert metadata to JSON
}

pub fn find_differences(previous: Report, current: Report) -> Vec<Difference> {
    let prev_fns = relevant_functions(&previous);
    let curr_fns = relevant_functions(&current);

    let mut old_map: HashMap<(String, u64, String), &ReportItem> = HashMap::new();

    for item in prev_fns {
        old_map.insert(
            (
                item.name.clone(),
                item.size,
                metadata_to_key(&item.metadata),
            ),
            item,
        );
    }

    let mut differences = Vec::new();

    for item in curr_fns {
        if let Some(&old_item) = old_map.get(&(
            item.name.clone(),
            item.size,
            metadata_to_key(&item.metadata),
        )) {
            if old_item.fuzzy_match_percent != item.fuzzy_match_percent {
                differences.push(Difference {
                    new_item: item.clone(),
                    old_item: old_item.clone(),
                });
            }
        }
    }

    differences
}

pub fn relevant_functions(report: &Report) -> Vec<&ReportItem> {
    // We only care about Bob code
    // But we might want to report on everything if we're looking at a PR...
    report
        .units
        .iter()
        .filter(|u| u.name.contains("/SB/"))
        .flat_map(|u| &u.functions)
        .collect()
}
