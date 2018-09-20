#[macro_use]
extern crate clap;
extern crate crc;

use clap::{App, AppSettings, Arg, SubCommand};

mod monero;

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
