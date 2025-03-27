use argp::FromArgs;

/// Oracle of Knowledge (OK)
///
/// I exist to compare decompilation reports.
#[derive(FromArgs, Debug)]
pub struct OKArgs {
    /// Path to the previous Objdiff report file
    #[argp(option)]
    pub previous: String,

    /// Path to the current Objdiff report file
    #[argp(option)]
    pub current: String,

    #[argp(subcommand)]
    pub action: Option<OKAction>,
}

#[derive(FromArgs, Debug)]
#[argp(subcommand)]
pub enum OKAction {
    PullRequest(PullRequest),
    PostToDiscord(PostToDiscord),
}

/// Process a Pull Request
#[derive(FromArgs, Debug)]
#[argp(subcommand, name = "pr")]
pub struct PullRequest {}

/// Post a progress report to Discord.
#[derive(FromArgs, Debug)]
#[argp(subcommand, name = "discord")]
pub struct PostToDiscord {
    /// Path to the progress commit data
    #[argp(option)]
    pub commit: String,

    /// Include a witty quip at the end of the progress report
    #[argp(switch)]
    pub add_commentary: bool,
}

// 1. compare two reports and create a struct containing differences
//      a) save differences to a file?
//      b) post differences to discord?
// 2. load and save progress report to a sensible json format, we only care about game
