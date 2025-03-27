mod args;
mod diff;

use args::OKArgs;
use diff::find_differences;
use objdiff_core::bindings::report::Report;

fn main() {
    let args: OKArgs = argp::parse_args_or_exit(argp::DEFAULT);

    let previous: Report = load_report(&args.previous);
    let current: Report = load_report(&args.current);

    let diffs = find_differences(previous, current);

    for diff in diffs {
        println!(
            "{}: {} {} ",
            diff.item.name, diff.old_fuzzy_match, diff.new_fuzzy_match
        );
    }

    if let Some(action) = args.action {
        println!("do action: {:?}", action);
    }
}

fn load_report(path: &str) -> Report {
    let bytes = std::fs::read(path).expect("Unable to read previous report file");
    Report::parse(&bytes).expect("Unable to parse the report! What have you done??")
}
