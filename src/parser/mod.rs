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

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::multispace1,
    combinator::value,
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

use crate::intermediate::{BrainFuck, Op, Stat};

/// Parse a brainfuck program from a source string.
pub fn parse(input: &str) -> Result<BrainFuck, &str> {
    match parse_stats(input) {
        Ok((rem, bf)) => {
            if rem.is_empty() {
                Ok(bf)
            } else {
                Err(rem)
            }
        }
        Err(nom::Err::Error(err)) => Err(err.input),
        _ => panic!(),
    }
}

/// Parse an assembly insert, retaining the text inside the insert
fn get_insert(input: &str) -> IResult<&str, Stat> {
    take_until("::")(input).map(|(rem, asm)| (rem, Stat::Asm(asm.to_string())))
}

/// Gte the inner
fn get_while(input: &str) -> IResult<&str, Stat> {
    parse_stats(input).map(|(rem, res)| (rem, Stat::WhileNonZero(res)))
}

/// Consume whitespace and comments.
fn get_whitespace(input: &str) -> IResult<&str, Vec<()>> {
    many0(alt((
        value((), multispace1),
        value((), tuple((tag("/*"), take_until("*/"), tag("*/")))),
    )))(input)
}

fn parse_stats(input: &str) -> IResult<&str, BrainFuck> {
    many0(delimited(
        get_whitespace,
        alt((
            value(Stat::PtrMove(1), tag(">")),
            value(Stat::PtrMove(-1), tag("<")),
            value(Stat::DerefOp(Op::Add, 1), tag("+")),
            value(Stat::DerefOp(Op::Add, -1), tag("-")),
            value(Stat::Input, tag(",")),
            value(Stat::Output, tag(".")),
            delimited(tag("::"), get_insert, tag("::")),
            delimited(tag("["), get_while, tag("]")),
        )),
        get_whitespace,
    ))(input)
}
