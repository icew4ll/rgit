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
    "pushdot" => pushdot(),
    "pulldot" => pulldot(),
    "pushall" => pushall(),
    _ => println!("none"),
});
// }}}
// all {{{
fn pushall() {
    pushdot();
    gitpush("/m/vim".to_string());
    gitpush("/m/dot".to_string());
    gitpush("/m/rgit".to_string());
}
// }}}
// push dot files to m/dot folder {{{
fn pushdot() {
    let home = dotenv!("HOME");
    let dot = format!("{}/m/dot", home);
    let nvim = format!("{}/.config/nvim/init.vim", home);
    let tmux = format!("{}/.tmux.conf.local", home);
    let ion = format!("{}/.config/ion/initrc", home);
    let alac = format!("{}/.config/alacritty/alacritty.yml", home);
    println!("{}", home);
    println!("{}", dot);
    println!("{}", nvim);
    println!("{}", tmux);
    println!("{}", ion);
    println!("{}", alac);
    let out = Exec::cmd("rsync")
        .arg("-av")
        .arg("-P")
        .arg(nvim)
        .arg(".")
        .cwd(&dot)
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
        .cwd(&dot)
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
        .cwd(&dot)
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
        .cwd(&dot)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()
        .expect("failed to execute")
        .stdout_str();
    println!("{}", out);
}
// }}}
// pull dot files to m/dot folder {{{
fn pulldot() {
    // Setup Paths
    let home = dotenv!("HOME");
    let dot = format!("{}/m/dot", home);
    let nvim = format!("{}/.config/nvim", home);
    let tmux = format!("{}", home);
    let ion = format!("{}/.config/ion", home);
    let alac = format!("{}/.config/alacritty", home);
    let dotnvim = format!("init.vim");
    let dottmux = format!(".tmux.conf.local");
    let dotion = format!("initrc");
    let dotalac = format!("alacritty.yml");
    println!("{}", home);
    println!("{}", dot);
    println!("{}", nvim);
    println!("{}", tmux);
    println!("{}", ion);
    println!("{}", alac);
    let out = Exec::cmd("rsync")
        .arg("-av")
        .arg("-P")
        .arg(dotnvim)
        .arg(".")
        .cwd(nvim)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()
        .expect("failed to execute")
        .stdout_str();
    println!("{}", out);
    let out = Exec::cmd("rsync")
        .arg("-av")
        .arg("-P")
        .arg(dottmux)
        .arg(".")
        .cwd(tmux)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()
        .expect("failed to execute")
        .stdout_str();
    println!("{}", out);
    let out = Exec::cmd("rsync")
        .arg("-av")
        .arg("-P")
        .arg(dotion)
        .arg(".")
        .cwd(ion)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()
        .expect("failed to execute")
        .stdout_str();
    println!("{}", out);
    let out = Exec::cmd("rsync")
        .arg("-av")
        .arg("-P")
        .arg(dotalac)
        .arg(".")
        .cwd(alac)
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
