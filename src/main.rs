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
//! BrainFuck compiler
//!
//! USAGE:
//!     bfc [OPTIONS] <FILE>
//!
//! ARGS:
//!     <FILE>    
//!
//! OPTIONS:
//!     -a, --after-cells <AFTER_CELLS>
//!             The number of byte cells to the right of the initial pointer position [default: 30000]
//!
//!     -b, --before-cells <BEFORE_CELLS>
//!             The number of byte cells to the left of the initial pointer position [default: 0]
//!
//!     -h, --help
//!             Print help information
//!
//!     -o, --outputpath <FILE>
//!             The name of the output file
//!
//!     -p, --print-result
//!             print the compilation result rather than writing to a file
//!
//!     -t, --target <TARGET>
//!             Set the target [default: interpreter] [possible values: interpreter, c99, arm]
//!
//!     -u, --unoptimised
//!             View the unoptimised intermediate representation
//!
//!     -V, --version
//!             Print version information
//! ```
//!
//! ## Exit Codes:
//! | Exit Code | Meaning                |
//! |-----------|------------------------|
//! | 0         | Successful Compilation |
//! | 1         | File Read Failure      |
//! | 2         | File Write Failure     |
//! | 3         | File Create Failure    |
//! | 100       | Syntax Error           |

#![feature(fn_traits)]
#![feature(fmt_internals)]
#![allow(dead_code)]

mod intermediate;
mod parser;
mod target;

use std::{
    fs::{read_to_string, File},
    io::Write,
    path::PathBuf,
    process::exit,
};

use clap::{ArgEnum, Parser};
use parser::parse;
use target::{compile, Backend};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Target {
    Interpreter,
    C99,
    Arm,
}

#[derive(Parser)]
#[clap(author = "Oliver Killane", about = "BrainFuck compiler" , long_about = Some("A brainfuck compiler targeting multiple architectures"), version = "0.0.1")]
struct Args {
    #[clap(parse(from_os_str), value_name = "FILE")]
    input_path: PathBuf,

    #[clap(
        short,
        long,
        parse(from_os_str),
        value_name = "FILE",
        help = "The name of the output file"
    )]
    output_path: Option<PathBuf>,

    #[clap(
        short,
        long,
        arg_enum,
        default_value_t = Target::Interpreter,
        help = "Set the target",
        value_name = "TARGET"
    )]
    target: Target,

    #[clap(
        short,
        long,
        default_value_t = 0,
        help = "The number of byte cells to the left of the initial pointer position"
    )]
    before_cells: u32,

    #[clap(
        short,
        long,
        default_value_t = 30_000,
        help = "The number of byte cells to the right of the initial pointer position"
    )]
    after_cells: u32,

    #[clap(short, long, help = "View the unoptimised intermediate representation")]
    unoptimised: bool,

    #[clap(
        short,
        long,
        help = "print the compilation result rather than writing to a file"
    )]
    print_result: bool,
}

const EXIT_SUCCESS: i32 = 0;
const FILE_READ_FAILURE: i32 = 1;
const FILE_WRITE_FAILURE: i32 = 2;
const FILE_CREATE_FAILURE: i32 = 3;
const SYNTAX_ERROR: i32 = 100;

fn main() {
    let Args {
        mut input_path,
        output_path,
        before_cells,
        after_cells,
        target,
        unoptimised,
        print_result,
    } = Args::parse();

    match read_to_string(input_path.clone()) {
        Ok(source) => {
            match parse(&source) {
                Ok(ir) => {
                    if unoptimised {
                        println!("Unoptimised intermediate representation:\n{}", ir)
                    }

                    // todo optimisation

                    if target == Target::Interpreter {
                        println!("Interpreter runs here")
                    } else {
                        let (result, ext) = compile(
                            match target {
                                Target::Interpreter => {
                                    panic!("Cannot set interpreter as compile backend")
                                }
                                Target::C99 => Backend::C99,
                                Target::Arm => unimplemented!(),
                            },
                            &ir,
                            before_cells,
                            after_cells,
                        );

                        if print_result {
                            println!("Compiler Result:\n{}", result)
                        } else {
                            let mut output_file = if let Ok(file) = match output_path {
                                Some(path) => File::create(path),
                                None => {
                                    input_path.set_extension(ext);
                                    File::create(input_path)
                                }
                            } {
                                file
                            } else {
                                exit(FILE_CREATE_FAILURE)
                            };

                            if write!(output_file, "{}", result).is_err() {
                                exit(FILE_WRITE_FAILURE);
                            }
                        }
                    }

                    exit(EXIT_SUCCESS)
                }
                Err(_err) => {
                    println!("An error occured");
                    exit(SYNTAX_ERROR)
                }
            }
        }
        Err(_) => {
            println!("Unable to open file");
            exit(FILE_READ_FAILURE)
        }
    }
}
