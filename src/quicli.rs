#[macro_use]
extern crate quicli;
use quicli::prelude::*;
// Add cool slogan for your app here, e.g.:
/// Get first n lines of a file
#[derive(Debug, StructOpt)]
struct Cli {
    // Add a CLI argument `--count`/-n` that defaults to 3, and has this help text:
    /// How many lines to get
    #[structopt(long = "git", short = "g")]
    git: String,
    // Add a positional argument that the user has to supply:
    /// Pass many times for more log output
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    verbosity: u8,
}
main!(|args: Cli, log_level: verbosity| {
    println!("{}", &args.git);
});
