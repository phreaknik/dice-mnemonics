# dice-nmeonics
A simple tool to help generate a mnemonic seed phrase using dice rolls as a source of entropy.

This program is written in Rust and assumes you have Rust tools installed. If not, please see Rust installation instructions here: https://www.rust-lang.org/install.html

## Build instructions
```
git@github.com:phreaknik/dice-mnemonics.git
cd dice-mnemonics
cargo build
```

## Usage example (Monero english dictionary)
```
./target/debug/dice-mnemonics -d dictionaries/monero-english.txt
```
