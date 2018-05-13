// extern crate csv;
extern crate csv;
extern crate subprocess;

use std::error::Error;
use std::fs::File;
use std::process;

use subprocess::{Exec, Redirection};

type Record = (String);

fn main() {
    let mut conn = Vec::new();
    // test function
    if let Err(err) = csv(&mut conn) {
        println!("{}", err);
        process::exit(1);
    }
    if let Err(err) = sub(&conn) {
        println!("{}", err);
        process::exit(1);
    }
}

fn sub(conn: &Vec<Record>) -> Result<(), Box<Error>> {
    println!("{:?}", conn);
    let pwd = "/home/ice/m/vim";
    let out = Exec::cmd("git")
        .arg("add")
        .arg("-A")
        .cwd(pwd)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()
        .expect("failed to execute")
        .stdout_str();
    println!("{}", out);
    let out2 = Exec::cmd("git")
        .arg("status")
        .cwd(pwd)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()
        .expect("failed to execute")
        .stdout_str();
    println!("{}", out2);
    let out3 = Exec::cmd("git")
        .arg("commit")
        .arg("-m")
        .arg("test")
        .cwd(pwd)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()
        .expect("failed to execute")
        .stdout_str();
    println!("{}", out3);
    let out3 = Exec::cmd("git")
        .arg("push")
        .cwd(pwd)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture()
        .expect("failed to execute")
        .stdout_str();
    println!("{}", out3);
    Ok(())
}

fn csv(conn: &mut Vec<Record>) -> Result<(), Box<Error>> {
    let file = File::open("conn.csv")?;
    let mut rdr = csv::ReaderBuilder::new().flexible(true).from_reader(file);
    for result in rdr.deserialize() {
        let record: Record = result?;
        conn.push(record);
    }
    Ok(())
}
