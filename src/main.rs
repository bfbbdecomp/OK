mod args;
mod commit;
mod pr;
mod website;

use std::{
    fs::File,
    io::{BufReader, Write},
};

use args::{OKAction, OKArgs};
use commit::Commit;
use objdiff_core::bindings::report::{Changes, Report};

use crate::{pr::generate_pr_report, website::AsmInfo};

fn main() {
    let args: OKArgs = argp::parse_args_or_exit(argp::DEFAULT);

    let changes = load_change_json(&args.changes);
    // let report = load_report_json(&args.report);

    let diff_json = serde_json::to_string_pretty(&changes).expect("Failed to serialize diffs");
    let mut file = File::create("diff.json").unwrap();
    file.write_all(diff_json.as_bytes()).unwrap();

    if let Some(action) = args.action {
        match action {
            OKAction::PullRequest(_) => {
                let pr_report = generate_pr_report(&changes);
                println!("{}", pr_report);
            }
            OKAction::PostToDiscord(_) => {
                // let commit = load_commit(&post_to_discord.commit);
                // println!("commit: {:?}", commit);
            }
            OKAction::BuildWebsite(args) => {
                let report = load_report_json(&args.report);
                let asm_info = load_asm_json(&args.asm_json);
                let first = asm_info.iter().nth(0).unwrap();
                println!("{:?}", first);
            }
        }
    }
}

fn load_change_json(path: &str) -> Changes {
    let json = std::fs::read_to_string(path).expect("Unable to read change file");
    serde_json::from_str(&json).unwrap()
}

fn load_report_json(path: &str) -> Report {
    let json = std::fs::read_to_string(path).expect("Unable to read change file");
    serde_json::from_str(&json).unwrap()
}

fn load_asm_json(path: &str) -> Vec<AsmInfo> {
    let json = std::fs::read_to_string(path).expect("Unable to read ASM json");
    serde_json::from_str(&json).unwrap()
}

fn load_commit(path: &str) -> Commit {
    let file = File::open(path).expect("Can't find progress commit file");
    let reader = BufReader::new(file);
    let commit: Commit = serde_json::from_reader(reader).expect("Failed parsing progress commit");
    commit
}
