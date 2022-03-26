//! The first brainfuck representation. It is extended over the grammar to allow
//! for optimisations and makes use of vectors to allow for slice pattern matches.

/// The brainfuck program consists of brainfuck stats.
pub type BrainFuck = Vec<Stat>;

/// Basic Integer operations
pub enum Op {
    Add,
    Mul,
    Div,
    Mod,
}

/// Basic Statement types
pub enum Stat {
    PtrMove(i32),
    DerefOp(Op, i32),
    Output,
    Input,
    WhileNonZero(BrainFuck),
    Asm(String),
}
