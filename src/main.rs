#[macro_use]
extern crate clap;
extern crate crc;

use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process;
use std::vec::Vec;
use crc::{crc32, Hasher32};

const DICE_SIDES: u32 = 6;

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
        ).arg(
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
        }
        _ => {
            println!("error: unable to determine mnemonic dictionary to use.");
            process::exit(1);
        }
    }
}

fn generate_mnemonic_monero(dict_file: &str) -> () {
    const DICT_SIZE: u32 = 1626;
    let mut word_indices: Vec<u32> = Vec::new();
    let mut trimmed_words = String::new();

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
    assert_eq!(dictionary.len(), 1626);

    // Calculate minimum # of rolls to preserve the most entropy
    let mut rem = DICT_SIZE;
    let mut min_rolls = 0;
    loop {
        rem = rem / DICE_SIDES as u32;

        if rem <= 0 {
            break;
        }

        min_rolls += 1;
    }

    // Get dice rolls from user and return dictionary word
    println!("Enter 'q' or 'quit' to exit");
    println!("Enter dice rolls (without spaces), to generate a seed words.");
    println!(
        "Use at least {} rolls to preserve {}% entropy.",
        min_rolls,
        100 * DICE_SIDES.pow(min_rolls) / DICT_SIZE
    );
    for i in 1..24 {
        print!("({}/25)\t", i);
        let input = prompt_user("")
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
            count += 1;

            // Calculate scale factor for this roll
            let scale_factor = DICT_SIZE / DICE_SIDES.pow(count);

            // Break if we have seen enough rolls
            if scale_factor <= 0 {
                break;
            }

            // Convert roll from char to number
            let roll: u32 = x.to_string().parse().expect("not a number");

            // Check this is a valid roll
            if roll > DICE_SIDES || roll < 1 {
                println!(
                    "Error: invalid die roll: {}. Must be a number between 1-6",
                    roll
                );
                break;
            }

            // Calculate next part of number
            num += (roll - 1) * scale_factor;
        }

        // Look up dictionary word and add to phrase
        word_indices.push(num);
        trimmed_words.push_str(&dictionary[num as usize][0..3]);
    }

    // Calculate checksum word
    let checksum = crc32::checksum_ieee(trimmed_words.as_bytes()) % word_indices.len() as u32;
    word_indices.push(checksum);

    // Print phrase
    for w in word_indices {
        println!("{}", dictionary[w as usize]);
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
