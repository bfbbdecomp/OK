use argp::FromArgs;

/// Oracle of Knowledge (OK)
///
/// I exist to compare decompilation reports.
#[derive(FromArgs, Debug)]
pub struct OKArgs {
    /// Path to the previous Objdiff report file
    #[argp(option)]
    pub changes: String,

    #[argp(subcommand)]
    pub action: Option<OKAction>,
}

#[derive(FromArgs, Debug)]
#[argp(subcommand)]
pub enum OKAction {
    PullRequest(PullRequest),
    BuildWebsite(BuildWebsite),
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

/// Build the files needed for the progress website
#[derive(FromArgs, Debug)]
#[argp(subcommand, name = "website")]
pub struct BuildWebsite {
    /// Path to the latest Objdiff report file
    #[argp(option)]
    pub report: String,

    /// Path to the cached assembly metadata
    #[argp(option)]
    pub asm_json: String,
}
