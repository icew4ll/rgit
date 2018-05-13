#[macro_use]
extern crate quicli;
use quicli::prelude::*;

/// Quicli proto
#[derive(Debug, StructOpt)]
struct Cli {
    git: String,
    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    verbosity: u8,
}
main!(|args: Cli, log_level: verbosity| {
    println!("{}", args.git);
});

// fn sub(conn: &Vec<Record>) -> Result<(), Box<Error>> {
//     println!("{:?}", conn);
//     let pwd = "/home/ice/m/vim";
//     let out = Exec::cmd("git")
//         .arg("add")
//         .arg("-A")
//         .cwd(pwd)
//         .stdout(Redirection::Pipe)
//         .stderr(Redirection::Merge)
//         .capture()
//         .expect("failed to execute")
//         .stdout_str();
//     println!("{}", out);
//     let out2 = Exec::cmd("git")
//         .arg("status")
//         .cwd(pwd)
//         .stdout(Redirection::Pipe)
//         .stderr(Redirection::Merge)
//         .capture()
//         .expect("failed to execute")
//         .stdout_str();
//     println!("{}", out2);
//     let out3 = Exec::cmd("git")
//         .arg("commit")
//         .arg("-m")
//         .arg("test")
//         .cwd(pwd)
//         .stdout(Redirection::Pipe)
//         .stderr(Redirection::Merge)
//         .capture()
//         .expect("failed to execute")
//         .stdout_str();
//     println!("{}", out3);
//     let out3 = Exec::cmd("git")
//         .arg("push")
//         .cwd(pwd)
//         .stdout(Redirection::Pipe)
//         .stderr(Redirection::Merge)
//         .capture()
//         .expect("failed to execute")
//         .stdout_str();
//     println!("{}", out3);
//     Ok(())
// }
