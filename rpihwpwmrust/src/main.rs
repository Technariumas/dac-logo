extern crate clap;
extern crate xyrust;
use clap::{Arg, App};
use std::process;

use xyrust::Config;

fn main() {
    let args = App::new("XY Rust")
        .version("1.0")
        .about("Does awesome things")
        .arg(
            Arg::with_name("input")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let config = Config { input: args.value_of("input").unwrap().to_owned() };

    if let Err(e) = xyrust::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
