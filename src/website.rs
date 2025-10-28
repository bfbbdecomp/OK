// We need to do some extra processing on the progress report
// before we build the website.
// We mostly use the objdiff report types, but with some added fields.

use argp::TopLevelCommand;
use objdiff_core::bindings::report::Report;
use serde::{Deserialize, Serialize};

pub fn generate_website_data(report: &Report, asm_info: &Vec<AsmInfo>) -> Report {
    let mut game_report = report.clone();
    game_report.categories = report
        .categories
        .iter()
        .filter(|c| c.id == "game")
        .map(|c| c.clone())
        .collect();

    game_report.calculate_progress_categories();

    game_report
}

pub struct WebsiteData {
    //
}

#[derive(Debug, Deserialize)]
pub struct AsmInfo {
    name: String,
    opcodes: Vec<String>,
    labels: Option<u32>,
}
