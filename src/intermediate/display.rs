//! Display for intermediate representation. Representing an extended version
//! of the brainfuck language.

use super::{BrainFuck, Op, Stat};
use std::{
    fmt::Display,
    i32::{MAX, MIN},
};

impl Display for BrainFuck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stat in &self.0 {
            write!(f, "{}", stat)?;
        }
        writeln!(f)
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Op::Add => "+",
                Op::Mul => "*",
                Op::Div => "/",
                Op::Mod => "%",
            }
        )
    }
}

impl Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stat::PtrMove(i) => match i {
                1 => write!(f, ">"),
                0 => write!(f, ""),
                -1 => write!(f, "<"),
                i @ MIN..=-2 => write!(f, "<({})", i),
                i @ 2..=MAX => write!(f, ">({})", i),
            },
            Stat::DerefOp(Op::Add, 1) => write!(f, "+"),
            Stat::DerefOp(Op::Add, -1) => write!(f, "-"),
            Stat::DerefOp(op, i) => write!(f, "{}({})", op, i),
            Stat::Output => write!(f, "."),
            Stat::Input => write!(f, ","),
            Stat::WhileNonZero(stats) => {
                write!(f, "[")?;
                for stat in stats {
                    write!(f, "{}", stat)?;
                }
                write!(f, "]")
            }
            Stat::Asm(asm) => write!(f, "::{}::", asm),
        }
    }
}
