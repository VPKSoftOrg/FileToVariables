use base64::prelude::*;
use clap::{command, Parser, Subcommand};
use std::fs::File;
use std::io::{Read, Write};
use std::str;

mod tests;

#[derive(Subcommand, Debug, Clone, PartialEq)]
enum Mode {
    /// Reads the variables from environment variables and writes them to a file.
    Read,
    /// Writes the variables from a file to output.
    Write,
}

#[derive(clap::ValueEnum, Debug, Clone)]
enum ShellFlavour {
    PowerShell,
    Bash,
    Cmd,
}

/// A simple program to encode data to base64 and decode it back.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File to read the data from.
    #[arg(short, long)]
    file: String,

    /// The mode for the data content. Allowed values: encrypt, decrypt, decrypt-string.
    ///
    /// encrypt: Encrypts the data content.
    ///
    /// decrypt: Decrypts the data content and returns the decrypted data in base64-format.
    ///
    /// decrypt-string: Decrypts the data content and returns the decrypted data in an UTF-8 string-format.
    #[command(subcommand)]
    mode: Mode,

    /// The format of the environment variable. E.g. "data_xyz{i}".
    #[arg(short, long)]
    var_format: String,

    /// The base64 block size. This indicates how many bytes are used in a base64 block.
    #[arg(short, long)]
    block: Option<u32>,

    /// The optional shell-flavour to use in case the data is written to an environment variable. Allowed values: powershell, cmd, bash.
    #[arg(long)]
    shell_flavour: Option<ShellFlavour>,
}

fn main() {
    let args = Args::parse();

    match args.mode {
        Mode::Read => {
            let data = output_from_environment(&args.var_format);
            let mut bin_data = Vec::new();
            for v in data {
                match BASE64_STANDARD.decode(v) {
                    Ok(v) => {
                        bin_data.extend_from_slice(&v);
                    }
                    Err(e) => panic!("{}", e),
                }
            }

            let mut file = match File::create(args.file) {
                Ok(file) => file,
                Err(e) => panic!("Error writing to file: {}", e),
            };

            match file.write_all(&bin_data) {
                Ok(_) => {}
                Err(e) => panic!("Error writing to file: {}", e),
            };
        }
        Mode::Write => {
            let mut file = File::open(args.file).unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            let data = buffer;

            match args.block {
                Some(block) => {
                    if block == 0 {
                        panic!("Please specify a block size greater than 0.");
                    }
                    let result =
                        output_variables(&data, &args.var_format, block, args.shell_flavour);
                    for v in result {
                        println!("{}", v);
                    }
                }
                None => panic!("Please specify the block size."),
            }
        }
    }
}

fn output_variables(
    data: &[u8],
    format: &str,
    base64_block_size: u32,
    shell_flavour: Option<ShellFlavour>,
) -> Vec<String> {
    let data = data.chunks(base64_block_size as usize);
    let mut i = 1;
    let mut result: Vec<String> = Vec::new();
    for v in data {
        let num = format!("{:0>2}", &i);
        let formatted = format.replace("{i}", &num);
        match shell_flavour {
            Some(ShellFlavour::PowerShell) => {
                result.push(format!(
                    "$Env:{}=\"{}\"",
                    formatted,
                    BASE64_STANDARD.encode(v)
                ));
            }
            Some(ShellFlavour::Cmd) => {
                result.push(format!(
                    "SET {}=\"{}\"",
                    formatted,
                    BASE64_STANDARD.encode(v)
                ));
            }
            Some(ShellFlavour::Bash) => {
                result.push(format!(
                    "EXPORT {}=\"{}\"",
                    formatted,
                    BASE64_STANDARD.encode(v)
                ));
            }
            None => {
                result.push(format!("{}={}", formatted, BASE64_STANDARD.encode(v)));
            }
        }
        i += 1;
    }
    result
}

fn output_from_environment(format: &str) -> Vec<String> {
    let mut i = 1;
    let mut result: Vec<String> = Vec::new();
    loop {
        let num = format!("{:0>2}", &i);
        let formatted = format.replace("{i}", &num);
        match std::env::var(&formatted) {
            Ok(v) => {
                result.push(v);
            }
            Err(_) => break,
        }
        i += 1;
    }
    result
}
