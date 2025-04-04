mod args;
mod commit;
mod diff;
mod pr;

use std::{
    fs::File,
    io::{BufReader, Write},
};

use args::{OKAction, OKArgs};
use commit::Commit;
use diff::find_differences;
use objdiff_core::bindings::report::Report;
use pr::PullRequestReport;

fn main() {
    let args: OKArgs = argp::parse_args_or_exit(argp::DEFAULT);

    let previous = load_report(&args.previous);
    let current = load_report(&args.current);

    let mut diffs = find_differences(previous.units, current.units);

    diffs.sections = diffs
        .sections
        .iter()
        // ghetto hack to remove this one text section which is showing up in all PRS:
        // might be related to this:
        // https://github.com/encounter/objdiff/issues/120#issuecomment-2770545367
        .filter(|x| !(x.unit_name == "main/SB/Game/zNPCTypeCommon" && x.name == ".text"))
        .map(|x| x.clone())
        .collect();

    let diff_json = serde_json::to_string_pretty(&diffs).expect("Failed to serialize diffs");
    let mut file = File::create("diff.json").unwrap();
    file.write_all(diff_json.as_bytes()).unwrap();

    if let Some(action) = args.action {
        match action {
            OKAction::PullRequest(_) => {
                let pr_report = PullRequestReport::new(diffs);
                println!("{}", pr_report.to_string());
            }
            OKAction::PostToDiscord(post_to_discord) => {
                let _ = load_commit(&post_to_discord.commit);
                //println!("commit: {:?}", commit);
            }
        }
    }
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
