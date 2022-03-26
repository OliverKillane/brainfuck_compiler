//! The parser for generating brainfuck from text.
//!
//! ## Basic grammar
//! ```text
//! <PtrLeft>  ::= '<'
//! <PtrRight> ::= '>'
//! <Inc>      ::= '+'
//! <Dec>      ::= '-'
//! <Input>    ::= ','
//! <Output>   ::= '.'
//! <While>    ::= '[' <Stat>* ']'
//! <ASM>   ::= '::' .* '::'
//! <Stat>     ::= <PtrLeft> | <PtrRight> | <Inc> | <Dec> | <Input> | <Output> | <While> | <ASM(s)>
//! ```
//!
//! ## Conversion:
//! When converting to the intermediate representation we can use:
//! ```
//! <PtrLeft> => Stat::PtrMove(1)
//! <PtrRight> => Stat::PtrMove(-1)
//! <Inc> => Stat::DerefOp(Op::Add, 1)
//! <Dec> => Stat::DerefOp(Op::Add, -1)
//! <Input> => Stat::Input
//! <Output> => Stat::Output
//! <While> => Stat::WhileNonZero(...)
//! <ASM> => Stat::Asm(...)
//! ```

use crate::intermediate::BrainFuck;

fn parse(_source: &str) -> BrainFuck {
    todo!()
}
