#[macro_use]
extern crate dotenv_codegen;

#[macro_use]
extern crate quicli;
extern crate chrono;
extern crate dotenv;
extern crate subprocess;
use chrono::prelude::*;
use quicli::prelude::*;
use subprocess::{Exec, Redirection};

/// IceSync
#[derive(Debug, StructOpt)]
struct Cli {
    git: String,
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    verbosity: u8,
}

// main {{{
main!(|args: Cli, log_level: verbosity| match args.git.as_ref() {
    "vim" => gitpush("/m/vim".to_string()),
    "dot" => gitpush("/m/dot".to_string()),
    "rgit" => gitpush("/m/rgit".to_string()),
    "rc" => rsync(),
    "all" => all(),
    _ => println!("none"),
});
// }}}
// all {{{
fn all() {
    rsync();
    gitpush("/m/vim".to_string());
    gitpush("/m/dot".to_string());
    gitpush("/m/rgit".to_string());
}
// }}}
// rsync {{{
fn rsync() {
    let home = dotenv!("HOME");
    let nvim = format!("{}/.config/nvim/init.vim", home);
    let tmux = format!("{}/.tmux.conf.local", home);
    let ion = format!("{}/.config/ion/initrc", home);
    let alac = format!("{}/.config/alacritty/alacritty.yml", home);
    let out = Exec::cmd("rsync")
        .arg("-av")
        .arg("-P")
        .arg(nvim)
        .arg(".")
        .cwd("/home/ice/m/dot/")
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()
        .expect("failed to execute")
        .stdout_str();
    println!("{}", out);
    let out = Exec::cmd("rsync")
        .arg("-av")
        .arg("-P")
        .arg(tmux)
        .arg(".")
        .cwd("/home/ice/m/dot/")
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()
        .expect("failed to execute")
        .stdout_str();
    println!("{}", out);
    let out = Exec::cmd("rsync")
        .arg("-av")
        .arg("-P")
        .arg(ion)
        .arg(".")
        .cwd("/home/ice/m/dot/")
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()
        .expect("failed to execute")
        .stdout_str();
    println!("{}", out);
    let out = Exec::cmd("rsync")
        .arg("-av")
        .arg("-P")
        .arg(alac)
        .arg(".")
        .cwd("/home/ice/m/dot/")
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()
        .expect("failed to execute")
        .stdout_str();
    println!("{}", out);
}
// }}}
// gitpush {{{
fn gitpush(pwd: String) {
    let utc: DateTime<Utc> = Utc::now();
    println!("{}", utc);
    let home = dotenv!("HOME");
    let pwd2 = format!("{}{}", home, &pwd);
    println!("{}", &pwd2);
    let out = Exec::cmd("git")
        .arg("add")
        .arg("-A")
        .cwd(&pwd2)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()
        .expect("failed to execute")
        .stdout_str();
    println!("{}", out);
    let out2 = Exec::cmd("git")
        .arg("status")
        .cwd(&pwd2)
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
        .cwd(&pwd2)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()
        .expect("failed to execute")
        .stdout_str();
    println!("{}", out3);
    let out3 = Exec::cmd("git")
        .arg("push")
        .cwd(&pwd2)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()
        .expect("failed to execute")
        .stdout_str();
    println!("{}", out3);
}
// }}}
