#[macro_use]
extern crate serde_derive;
extern crate envy;

#[macro_use]
extern crate quicli;
extern crate chrono;
extern crate subprocess;
use chrono::prelude::*;
use quicli::prelude::*;
use subprocess::{Exec, Redirection};

/// Quicli proto
#[derive(Debug, StructOpt)]
struct Cli {
    git: String,
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    verbosity: u8,
}

#[derive(Deserialize, Debug)]
struct Config {
  home: String,
}


main!(|args: Cli, log_level: verbosity| match args.git.as_ref() {
    "vim" => gitpush("/home/ice/m/vim".to_string()),
    "dot" => gitpush("/home/ice/m/dot".to_string()),
    "rgit" => gitpush("/home/ice/m/rgit".to_string()),
    "all" => all(),
    _ => println!("none"),
});

fn all() {
    gitpush("/home/ice/m/vim".to_string());
    gitpush("/home/ice/m/dot".to_string());
    gitpush("/home/ice/m/rgit".to_string());
}

// gitpush {{{
fn gitpush(pwd: String) {
    match envy::from_env::<Config>() {
       Ok(config) => dirify(config.home),
       Err(error) => panic!("{:#?}", error),
    }
    fn dirify(dir: String) {
       println!("{}", dir);
    }
    let utc: DateTime<Utc> = Utc::now();
    println!("{}", utc);
    let out = Exec::cmd("git")
        .arg("add")
        .arg("-A")
        .cwd(&pwd)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()
        .expect("failed to execute")
        .stdout_str();
    println!("{}", out);
    let out2 = Exec::cmd("git")
        .arg("status")
        .cwd(&pwd)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()
        .expect("failed to execute")
        .stdout_str();
    println!("{}", out2);
    let out3 = Exec::cmd("git")
        .arg("commit")
        .arg("-m")
        .arg(utc.to_string())
        .cwd(&pwd)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()
        .expect("failed to execute")
        .stdout_str();
    println!("{}", out3);
    let out3 = Exec::cmd("git")
        .arg("push")
        .cwd(&pwd)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()
        .expect("failed to execute")
        .stdout_str();
    println!("{}", out3);
}
// }}}
