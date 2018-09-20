#[macro_use]
extern crate clap;
extern crate crc;

use clap::{App, AppSettings, Arg, SubCommand};

mod monero;

const BANNER: &str = "______ _____ _____ ___________ _   _______  ___  _____ _____
|  _  \\_   _/  __ \\  ___| ___ \\ | | | ___ \\/ _ \\/  ___|  ___|
| | | | | | | /  \\/ |__ | |_/ / |_| | |_/ / /_\\ \\ `--.| |__  
| | | | | | | |   |  __||  __/|  _  |    /|  _  |`--. \\  __| 
| |/ / _| |_| \\__/\\ |___| |   | | | | |\\ \\| | | /\\__/ / |___ 
|___/  \\___/ \\____|____/\\_|   \\_| |_|_| \\_\\_| |_|____/\\____/";

fn main() {
    // Setup command-line interface (CLI)
    let arg_matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("monero")
                .about("Generate Monero style mnemonic seed phrase.")
                .arg(
                    Arg::with_name("help")
                        .short("h")
                        .long("help")
                        .help("Prints help information")
                        .takes_value(true),
                ).arg(
                    Arg::with_name("dict-path")
                        .short("p")
                        .long("dict-path")
                        .value_name("DICT_FILE")
                        .help("Path to dictionary file")
                        .takes_value(true),
                ),
        ).get_matches();

    // Print banner
    println!("\n\n\n");
    println!("===============================================================");
    println!("\n{}\n", BANNER);
    println!("===============================================================");
    println!("");
    println!("Mode: {}", arg_matches.subcommand_name().unwrap());

    // Determine which subcommand was chosen
    match arg_matches.subcommand_name() {
        // Some("monero") => println!("{:?}", cli_matches.subcommand_matches("monero"),
        Some("monero") => monero::run(arg_matches.subcommand_matches("monero")),
        _ => {
            println!("error: No subcommand provided.");
            println!("Run 'rusty-math -h' for a list of available commands.");
        }
    }
}
