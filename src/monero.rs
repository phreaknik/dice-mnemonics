use clap::ArgMatches;
use crc::crc32;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process;
use std::vec::Vec;

const DICE_SIDES: usize = 6;
const NUM_ROLLS: usize = 4;
const DICT_SIZE: usize = 1626;
const DEFAULT_DICTIONARY: &str = "dictionaries/monero-english.txt";
const DICTIONARY_OFFSET: usize = 13;

pub fn run(args: Option<&ArgMatches>) -> () {
    let mut word_indices: Vec<usize> = Vec::new();
    let mut trimmed_words = String::new();

    let f = File::open(
        args.unwrap()
            .value_of("dict-path")
            .unwrap_or(DEFAULT_DICTIONARY),
    ).unwrap_or_else(|err| {
        println!("error: Error opening dictionary file: {}", err);
        process::exit(1);
    });

    // Build dictionary from each line of dictionary file
    let dictionary: Vec<String> = BufReader::new(f)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    assert_eq!(dictionary.len(), DICT_SIZE);

    // Get dice rolls from user and return dictionary word
    println!("Enter {} dice rolls (without spaces), to generate each seed word.", NUM_ROLLS);
    println!("Enter 'q' or 'quit' to exit");
    println!("\n");
    
    // Get first 24 words
    let mut current_word = 1;
    loop {
        // Prompt user to enter dice rolls
        let mut rolls: Vec<usize> = Vec::new();
        print!("({}/24)\t", current_word);
        if let Ok(input) = prompt_user("") {
            match &input as &str {
                // Detect quit command
                "q" | "Q" => process::exit(0),

                // Convert input to vector of numbers
                _ => {
                    rolls = input
                        .chars()
                        .map(|c| c.to_string().parse().expect("not a number"))
                        .collect()
                }
            }
        } else {
            println!("error: invalid input");
            continue;
        }

        // Loop for each roll to calculate large number
        let mut num = 0;
        let mut count = 0;
        for roll in rolls {
            // Calculate scale factor for this roll
            let scale_factor = DICT_SIZE / DICE_SIDES.pow(count + 1);

            // Break if we have seen enough rolls
            if scale_factor <= 0 {
                break;
            }

            // Check this is a valid roll
            if roll > DICE_SIDES || roll < 1 {
                println!(
                    "error: invalid die roll: {}. Rolls must be between between 1-{}",
                    roll, DICE_SIDES
                );
                break;
            }

            // Calculate next part of number
            num += (roll - 1) * scale_factor;

            // Increment counter
            count += 1;
        }

        // Make sure all 4 rolls were summed
        if count < NUM_ROLLS as u32 {
            // Skip this word, since less than 4 dice were used
            println!(
                "error: roll at least {} valid dice to preserve entropy",
                NUM_ROLLS
            );
            continue;
        }

        // Add an offset to our dictionary lookup. This helps differentiate
        // the word set this algorithm spans from the word-set other offline
        // mnemonic phrase generators may span.
        num += DICTIONARY_OFFSET;
        
        // Look up dictionary word and add to phrase
        word_indices.push(num);
        trimmed_words.push_str(&dictionary[num as usize][0..3]);

        // Check if we have 24 words yet
        current_word += 1;
        if current_word > 24 {
            break;
        }
    }

    // Calculate checksum word
    let checksum = crc32::checksum_ieee(trimmed_words.as_bytes()) as usize;
    let checkword_idx = word_indices[checksum % word_indices.len()];
    word_indices.push(checkword_idx);

    // Print phrase
        println!("\n\n\n");
    println!("===============================================================");
        println!("\t\t\tMonero Seed Phrase");
    println!("===============================================================");
    let mut idx = 1;
    for w in word_indices {
        println!("{}:\t{}", idx, dictionary[w as usize]);
        idx += 1;
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
