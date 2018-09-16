#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io::{self, Write, BufRead, BufReader};
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

    // Get dice rolls from user and return dictionary word
     println!("Enter up to 4 dice rolls (without spaces), to generate a seed word:");
    loop {
        let input = prompt_user("> ").unwrap_or_else(|err| {
            println!("error: {}", err);
            process::exit(1);
            })
            .to_string();

        // Check for quit signal
        match input.as_ref() {
            "q" | "Q" | "quit" => break,
            _ => (),
        }

        // Loop for each roll to calculate large number
        let rolls: Vec<char> = input.chars().collect();
        let mut num = 0;
        let mut count = 0;
        let dict_size = dictionary.len() - 1;
        let dice_sides = 6;
        for x in rolls {
            count = count + 1;

            // Break if we have seen enough rolls
            if count > 4 {
                break;
            }

            // Convert roll from char to number
            let roll: usize = x.to_string().parse().expect("not a number");

            // Check this is a valid roll
            if roll > dice_sides || roll < 1 {
                println!("Error: invalid die roll: {}. Must be a number between 1-6", roll);
                break;
            }
            
            // Calculate next part of number
            num = num + (roll - 1) * (dict_size / dice_sides.pow(count));
        }

        // Print dictionary word corresponding to the large number
        if num > dict_size {
            println!("error: dictionary overflow");
            process::exit(1);
        }
        println!("{}", dictionary[num]);
    }
}

fn prompt_user(msg: &str) -> (Result<String, String>) {
    // Print message
    print!("{}", msg);
    io::stdout().flush().unwrap();

    // Read input
    let mut s = String::new();
    if let Err(error) = io::stdin().read_line(&mut s) {
        return Err(error.to_string());
    }

    // Strip off newline or carriage return characters
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    // Make sure input is non-empty
    if s.len() == 0 {
        return Err("No input detected.".to_string());
    } else {
        return Ok(s);
    }
}

