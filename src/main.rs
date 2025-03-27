use argp::FromArgs;

/// Reach new heights
#[derive(FromArgs, Debug)]
struct GoUp {
    /// Foo
    #[argp(switch, short = 'j')]
    jump: bool,

    /// How high to go.
    #[argp(option)]
    height: usize,

    /// An optional nickname for the pilot.
    #[argp(option)]
    pilot_nickname: Option<String>,
}

fn main() {
    let up: GoUp = argp::parse_args_or_exit(argp::DEFAULT);
    println!("{:?}", up);
}
