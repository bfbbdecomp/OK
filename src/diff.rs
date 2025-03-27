use objdiff_core::bindings::report::{Report, ReportItem};

#[derive(Debug)]
pub struct Difference {
    previous: ReportItem,
    current: ReportItem,
}

pub fn find_differences(previous: Report, current: Report) -> Vec<Difference> {
    let prev_fns = relevant_functions(&previous);
    let curr_fns = relevant_functions(&current);

    for f in curr_fns {
        let meta = f.metadata.clone().unwrap();
        if meta.demangled_name.is_none() {
            println!("{:?}", f);
        }
    }
    /*
    let diffs: Vec<Difference> = prev_fns
    .iter()
    .filter(|f| {
        let curr = curr_fns.iter().find(|x| x.metadata.unwrap().demangled_name)
        f.fuzzy_match_percent
    });
    */

    vec![]
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
