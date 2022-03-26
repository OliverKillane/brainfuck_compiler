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
//! <ASM>      ::= '::' .* '::'
//! <Comment>  ::= '/*' .* '*/'
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


use nom::{IResult, bytes::complete::{tag, take_until}, combinator::value, branch::alt, multi::many0, sequence::delimited, character::complete::multispace0};

use crate::intermediate::{BrainFuck, Stat, Op};

pub fn parse(input: &str) -> Result<BrainFuck, &str> {
    match parse_stats(input) {
        Ok((_, bf)) => Ok(bf),
        Err(nom::Err::Error(err)) => Err(err.input),
        _ => panic!()
    }
}

fn get_insert(input: &str) -> IResult<&str, Stat> {
    let (rem, asm) = take_until("::")(input)?;
    Ok((rem, Stat::Asm(asm.to_string())))
}

fn get_while(input: &str) -> IResult<&str, Stat> {
    let (rem, res) = parse_stats(input)?;
    Ok((rem, Stat::WhileNonZero(res)))
}

fn parse_stats(input: &str) -> IResult<&str, BrainFuck> {
    many0(
        delimited(
            multispace0, 
            alt(
                (
                    value(Stat::PtrMove(1), tag(">")),
                    value(Stat::PtrMove(-1), tag("<")),
                    value(Stat::DerefOp(Op::Add, 1), tag("+")),
                    value(Stat::DerefOp(Op::Add, -1), tag("-")),
                    value(Stat::Input, tag(",")),
                    value(Stat::Output, tag(".")),
                    delimited(tag("::"), get_insert, tag("::")),
                    delimited(tag("["), get_while , tag("]"))
                )
            ),
            multispace0
        )
    )
    (input)
}
