// We need to do some extra processing on the progress report
// before we build the website.
// We mostly use the objdiff report types, but with some added fields.

use objdiff_core::bindings::report::Report;
use serde::{Deserialize, Serialize};
// use ts_rs::TS;

#[derive(Debug, Deserialize)]
// #[ts(export)]
pub struct AsmInfo {
    name: String,
    opcodes: Vec<String>,
    labels: Option<u32>,
}
