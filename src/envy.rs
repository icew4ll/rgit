#[macro_use]
extern crate serde_derive;
extern crate envy;

#[derive(Deserialize, Debug)]
struct Config {
  home: String,
}

fn main() {
    match envy::from_env::<Config>() {
       Ok(config) => println!("{:#?}", config),
       Err(error) => panic!("{:#?}", error),
    }
}
