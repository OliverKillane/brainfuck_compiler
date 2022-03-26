//! # BrainFuck  Compiler Project
//!
//! ## Description
//! This project aims to create a basic brainfuck compiler to target multiple
//! architectures, allowing for the basic brainfuck syntax (and an infinite
//! number of cells (in rightward direction)), as well as brainfuck with
//! assembly inserts to allow for more complex programs (e.g writing basic
//! operating systems).
//!
//! ## Structure
//! 1. Parse brainfuck into an intermediate representation
//! 2. Apply optimisations on the intermediate representation, allowing for
//!    patterns to be matched and replaced with assembly inserts (optimisation)
//! 3. Convert the intermediate representation to the target language (e.g
//!    x86 assembly), or if the target is the interpreter - interpret.
//! 
//! ## Command Line Interface:
//! ```text
//! brainfuck_compiler 0.0.1
//! Oliver Killane
//! A brainfuck compiler targeting multiple architectures
//! 
//! USAGE:
//!     bf [OPTIONS] <FILE>
//! 
//! ARGS:
//!     <FILE>
//! 
//! 
//! OPTIONS:
//!     -h, --help
//!             Print help information
//! 
//!     -o, --outputpath <FILE>
//!             The name of the output file
//! 
//!     -V, --version
//!             Print version information
//! ```

mod arch;
mod intermediate;
mod parser;

use std::{
    fs::{read_to_string, File},
    io::Write,
    path::PathBuf,
    process::exit,
};

use clap::Parser;
#[derive(Parser)]
#[clap(author = "Oliver Killane", about = "BrainFuck compiler" , long_about = Some("A brainfuck compiler targeting multiple architectures"), version = "0.0.1")]
struct Args {
    #[clap(parse(from_os_str), value_name = "FILE")]
    inputpath: PathBuf,

    #[clap(
        short,
        long,
        parse(from_os_str),
        value_name = "FILE",
        help = "The name of the output file"
    )]
    outputpath: Option<PathBuf>,
}

const EXIT_SUCCESS: i32 = 0;
const FILE_READ_FAILURE: i32 = 1;
const FILE_WRITE_FAILURE: i32 = 2;
const FILE_CREATE_FAILURE: i32 = 3;
const SYNTAX_ERROR: i32 = 4;

fn main() {
    let Args {
        mut inputpath,
        outputpath,
    } = Args::parse();

    match read_to_string(inputpath.clone()) {
        Ok(_source) => {
            // todo parsing

            // todo optimisation

            // todo generation

            let mut outputfile = if let Ok(file) = match outputpath {
                Some(path) => File::create(path),
                None => {
                    inputpath.set_extension(".s");
                    File::create(inputpath)
                }
            } {
                file
            } else {
                exit(FILE_CREATE_FAILURE)
            };

            if outputfile.write("this is the code".as_bytes()).is_err() {
                exit(FILE_WRITE_FAILURE);
            }

            exit(EXIT_SUCCESS)
        }
        Err(_) => {
            println!("Unable to open file");
            exit(FILE_READ_FAILURE)
        }
    }
}
