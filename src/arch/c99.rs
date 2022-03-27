//! Transpilation to C.

use std::{i32::{MIN, MAX}, fmt::Formatter};

use crate::intermediate::{BrainFuck, Stat, Op};

/// Compile the brainfuck program, given pre bytes of memory before the 
/// starting position, and post bytes after.
pub fn compile(BrainFuck(stats): &BrainFuck, pre: u32, post: u32) -> String {

    let mut result = String::new();
    let mut f = Formatter::new(&mut result);

    writeln!(f, "#include <stdio.h>
int main(int argc, char **argv) {{
    char cells[{}] = {{0}};
    char* ptr = cells{};", pre + post, if pre == 0 {"".to_string()} else {pre.to_string()}).expect("Writing to string");

    for stat in stats {
        transpile_stat(stat, 1, &mut f).expect("Writing to string");
    }

    writeln!(f, "\n}}").expect("Writing to string");

    result
}

fn transpile_stat(stat: &Stat, indent_lvl: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
    let indent = "\t".repeat(indent_lvl);
    write!(f, "{}", indent)?;
    match stat {
        Stat::PtrMove(i @ MIN..=-1) => writeln!(f, "ptr -= {};", i),
        Stat::PtrMove(i @ 1..=MAX) => writeln!(f, "ptr += {};", i),
        Stat::PtrMove(0) => writeln!(f, "/* redundant ptr move*/"),
        Stat::DerefOp(op, i) => writeln!(f, "ptr {}= {};", match op {
                Op::Add => "+",
                Op::Mul => "*",
                Op::Div => "/",
                Op::Mod => "%",
            }, i),
        Stat::Output => write!(f, "putchar(*ptr);"),
        Stat::Input => write!(f, "*ptr = getchar();"),
        Stat::WhileNonZero(stats) => {
            writeln!(f, "while(*ptr) {{")?;
            for stat in stats {
                transpile_stat(stat, indent_lvl + 1, f)?;
            }
            writeln!(f, "{}}}", indent)
        },
        Stat::Asm(asm) => {
            write!(f, "/* Inserted Section*/\n{}", indent)?;
            for c in asm.chars() {
                if c == '\n' {
                    write!(f, "\n{}", indent)?;
                } else {
                    write!(f, "{}", c)?;
                }
            }
            writeln!(f)
        },
    }
}