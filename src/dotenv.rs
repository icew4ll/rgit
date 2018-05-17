#[macro_use]
extern crate dotenv_codegen;

extern crate dotenv;

fn main() {
  println!("{}", dotenv!("HOME"));
}
