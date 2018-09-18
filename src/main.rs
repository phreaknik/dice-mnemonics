#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process;
use std::vec::Vec;

const DICE_SIDES: usize = 6;

fn main() {
    let arg_matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("type")
                .short("t")
                .long("type")
                .value_name("MNEMONIC_TYPE")
                .help("What type of mnemonic phrase are you generating? Eg. 'monero-english'")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("dictionary")
                .short("d")
                .long("dictionary")
                .value_name("DICT_FILE")
                .help("Path to dictionary file.")
                .takes_value(true),
        ).get_matches();
    
    // Which type of mnemonic phrase are we generating?
    match arg_matches.value_of("type").unwrap() {
        "monero-english" => {
                if let Some(dict_file) = arg_matches.value_of("dictionary") {
                    generate_mnemonic_monero(&dict_file);
                } else {
                    generate_mnemonic_monero("dictionaries/monero-english.txt");
                }
                
            },
        _ => { 
                println!("error: unable to determine mnemonic dictionary to use.");
                process::exit(1);
            }
    }

}

fn generate_mnemonic_monero(dict_file: &str) -> () {
    // Open the dictionary file for reading
    let f = File::open(dict_file).unwrap_or_else(|err| {
        println!("error: {}", err);
        process::exit(1);
    });

    // Build dictionary from each line of dictionary file
    let dictionary: Vec<String> = BufReader::new(f)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    let dict_size = dictionary.len() - 1;

    // Calculate minimum # of rolls to preserve the most entropy
    let mut rem = dict_size;
    let mut min_rolls = 0;
    loop {
        rem = rem / DICE_SIDES;

        if rem <= 0 {
            break;
        }

        min_rolls = min_rolls + 1;
    }

    // Get dice rolls from user and return dictionary word
    println!("Enter 'q' or 'quit' to exit");
    println!("Enter dice rolls (without spaces), to generate a seed words.");
    println!(
        "Use at least {} rolls to preserve {}% entropy.",
        min_rolls,
        100 * DICE_SIDES.pow(min_rolls) / dict_size
    );
    loop {
        let input = prompt_user("> ")
            .unwrap_or_else(|err| {
                println!("error: {}", err);
                process::exit(1);
            }).to_string();

        // Check for quit signal
        match input.as_ref() {
            "q" | "Q" | "quit" => break,
            _ => (),
        }

        // Make sure we have enough rolls to preserve maximum entropy
        let rolls: Vec<char> = input.chars().collect();
        if (rolls.len() as u32) < min_rolls {
            println!(
                "error: roll at least {} dice to preserve entropy",
                min_rolls
            );
            continue;
        }

        // Loop for each roll to calculate large number
        let mut num = 0;
        let mut count = 0;
        for x in rolls {
            count = count + 1;

            // Calculate scale factor for this roll
            let scale_factor = dict_size / DICE_SIDES.pow(count);

            // Break if we have seen enough rolls
            if scale_factor <= 0 {
                break;
            }

            // Convert roll from char to number
            let roll: usize = x.to_string().parse().expect("not a number");

            // Check this is a valid roll
            if roll > DICE_SIDES || roll < 1 {
                println!(
                    "Error: invalid die roll: {}. Must be a number between 1-6",
                    roll
                );
                break;
            }

            // Calculate next part of number
            num = num + (roll - 1) * scale_factor;
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
