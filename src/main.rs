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

mod arch;
mod intermediate;
mod parser;

fn main() {
    todo!()
}
