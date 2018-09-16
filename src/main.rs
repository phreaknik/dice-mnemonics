#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use std::vec::Vec;

fn main() {
    let arg_matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("dictionary")
                .short("d")
                .long("dictionary")
                .value_name("DICT_FILE")
                .help("Path to dictionary file.")
                .takes_value(true)
                .required(true),
        ).get_matches();

    // Open the dictionary file for reading
    let path = arg_matches.value_of("dictionary").unwrap();
    let dict_file = File::open(path).unwrap_or_else(|err| {
        println!("error: {}", err);
        process::exit(1);
    });

    // Build dictionary from each line of dictionary file
    let dictionary: Vec<String> = BufReader::new(dict_file)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    
}
