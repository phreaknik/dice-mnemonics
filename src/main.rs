#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process;
use std::vec::Vec;

const DICE_SIDES: u32 = 6;

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
    let dict_size: u32 = dictionary.len() as u32;

    // Calculate optimal dice combination to maximize entropy.
    // This is achieved by decomposing the dictionary size into
    // factors of base-<DICE_SIDES> numbers.
    let mut base_digits = 0;
    let mut max_num = 1;
    while max_num < dict_size {
        base_digits = base_digits + 1;
        max_num = max_num * DICE_SIDES;
    }

    // Calculate radix decomposition of dictionary size
    let mut radix_decomp: Vec<u32> = Vec::new();
    let mut quo = dict_size;
    let mut rem = dict_size;
    for i in (1..base_digits).rev() {
        quo = rem / DICE_SIDES.pow(i); // Calculate running quotient
        rem = rem - quo*DICE_SIDES.pow(i); // Calculate remainder
        radix_decomp.push(quo); // Add new digit to radix decomposition
    }

    // Calculate optimal number of dice to roll
    let mut optimal_num_dice = 0;
    let mut count = 0;
    for n in radix_decomp.iter().rev() {
        count = count + 1;
        optimal_num_dice += n * count;
    }

    // Print optimal dice strategy
    println!{"Roll {} {}-sided dice to preserve 100% of dictionary entropy.", optimal_num_dice, DICE_SIDES};

   // Print basic usage instructions
    println!("Enter dice rolls (without spaces), to generate a seed words.");
    println!("Enter 'q' or 'quit' to exit");

     // Get dice rolls from user and return dictionary word
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
        let rolls: Vec<u8> = input.chars()
                                    .map(|c| {
                                        c.to_string()
                                            .parse()
                                            .expect("Not a number")
                                    }).collect();
        if (rolls.len() as u32) < optimal_num_dice {
            println!(
                "error: roll at least {} dice to preserve entropy",
                optimal_num_dice
            );
            continue;
        }

        // Compose large number
        let  num = calc_from_dice(2, vec![1, 1, 1, 1], vec![0, 0, 0]);
        // let  num = calc_from_dice(DICE_SIDES, radix_decomp.clone(), rolls);



        // let mut num = 0;
        // let mut order = radix_decomp.len() as u32;
        // println!("{:?}", radix_decomp);
        // for i in 0..order {
        // println!("loop 1");
        //     let n = radix_decomp[i as usize];
        //     for _ in 0..n {
        //         println!("loop 2");
        //         for k in (0..order).rev() {
        //             println!("loop 3"); 
        //             let r = rolls.pop().unwrap();
        //             let roll: u32 = r.to_string().parse().expect("not a number");
        //             num = num + ((roll - 1) * DICE_SIDES.pow(k));
        //             println!("num = {}, roll = {}, order = {}, 6^k = {}",
        //                 num, roll, order, DICE_SIDES.pow(k)
        //             )
        //         }
        //     }
        //     order = order - 1;
        // }

        // Print dictionary word corresponding to the large number
        if num > dict_size {
            println!("error: dictionary overflow");
            process::exit(1);
        }
        println!("{}", num);
        println!("{}", dictionary[num as usize]);
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

fn calc_from_dice(radix: u32, pattern: Vec<u32>, rolls: Vec<u8>) -> u32 {
    let mut order = 0;
    let mut roll = 0;
    let mut num = 0;

    let mut first = true;
    print!("(");
    for n in pattern {
        if !first {
            print!(" + ");
        }

        for k in (0..order).rev() {
            if !first {
                print!(" + ");
            }

            let mut coeff = 0;
            for _ in 0..n {
                coeff += rolls[roll] as u32;
                roll += 1;
                if first {
                    print!("{}", rolls[roll]);
                } else {
                    print!(" + {}", rolls[roll]);
                }
                first = false;
            }

            num += coeff * radix.pow(k);

            print!(")*({}^{})", radix, k);
        }
        order += 1;
        print!(")");
    }
    print!(")");
    num
}