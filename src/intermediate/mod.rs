//! The first brainfuck representation. It is extended over the grammar to allow
//! for optimisations and makes use of vectors to allow for slice pattern matches.

#[derive(Debug, Clone, PartialEq)]
pub struct BrainFuck(pub Stats);

/// The brainfuck program consists of brainfuck stats.
pub type Stats = Vec<Stat>;

/// Basic Integer operations
#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add,
    Mul,
    Div,
    Mod,
}

/// Basic Statement types
#[derive(Debug, Clone, PartialEq)]
pub enum Stat {
    PtrMove(i32),
    DerefOp(Op, i32),
    Output,
    Input,
    WhileNonZero(Stats),
    Asm(String),
}
