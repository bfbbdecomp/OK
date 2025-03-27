mod args;
mod commit;
mod diff;

use std::{
    fs::{File, read},
    io::BufReader,
    process::Command,
};

use args::OKArgs;
use commit::Commit;
use diff::find_differences;
use objdiff_core::bindings::report::Report;

fn main() {
    let args: OKArgs = argp::parse_args_or_exit(argp::DEFAULT);

    let previous = load_report(&args.previous);
    let current = load_report(&args.current);
    let commit = load_commit(&args.commit);

    let diffs = find_differences(previous.units, current.units);

    for diff in diffs.functions {
        println!(
            "{} {} {} {} ",
            diff.unit, diff.old.name, diff.old.fuzzy_match_percent, diff.new.fuzzy_match_percent
        );
    }
    println!("SECTIONS:");
    for diff in diffs.sections {
        println!(
            "{} {} {:?} {:?} ",
            diff.unit, diff.old.name, diff.old, diff.new
        );
    }

    if let Some(action) = args.action {
        println!("do action: {:?}", action);
    }

    println!("{:?}", commit);
}

fn load_report(path: &str) -> Report {
    let bytes = std::fs::read(path).expect("Unable to read previous report file");
    Report::parse(&bytes).expect("Unable to parse the report! What have you done??")
}

fn load_commit(path: &str) -> Commit {
    let file = File::open(path).expect("Can't find progress commit file");
    let reader = BufReader::new(file);
    let commit: Commit = serde_json::from_reader(reader).expect("Failed parsing progress commit");
    commit
}
