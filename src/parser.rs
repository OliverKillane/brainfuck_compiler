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
//! <Comment>  ::= '#' .* '#'
//! <Stat>     ::= <PtrLeft> | <PtrRight> | <Inc> | <Dec> | <Input> | <Output> | <While> | <ASM>
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

use crate::intermediate::{BrainFuck, Op, Stat, Stats};

/// Parse a brainfuck program from a source string.
pub fn parse(input: &str) -> Result<BrainFuck, &str> {
    match parse_stats(input) {
        Ok((rem, bf)) => {
            if rem.is_empty() {
                Ok(BrainFuck(bf))
            } else {
                Err(rem)
            }
        }
        Err(nom::Err::Error(err)) => Err(err.input),
        _ => panic!(),
    }
}

/// Parse an assembly insert, placing the text inside into an assembly insert
/// statement.
fn get_insert(input: &str) -> IResult<&str, Stat> {
    delimited(tag("::"), take_until("::"), tag("::"))(input)
        .map(|(rem, asm)| (rem, Stat::Asm(asm.to_string())))
}

/// Get the inner instructions of a basic while loop
fn get_while(input: &str) -> IResult<&str, Stat> {
    delimited(tag("["), parse_stats, tag("]"))(input)
        .map(|(rem, res)| (rem, Stat::WhileNonZero(res)))
}

/// Consume whitespace and comments.
fn get_whitespace(input: &str) -> IResult<&str, Vec<()>> {
    many0(alt((
        value((), multispace1),
        value((), tuple((tag("#"), take_until("#"), tag("#")))),
    )))(input)
}

/// Parse statements into a vector of statements
fn parse_stats(input: &str) -> IResult<&str, Stats> {
    many0(delimited(
        get_whitespace,
        alt((
            value(Stat::PtrMove(1), tag(">")),
            value(Stat::PtrMove(-1), tag("<")),
            value(Stat::DerefOp(Op::Add, 1), tag("+")),
            value(Stat::DerefOp(Op::Add, -1), tag("-")),
            value(Stat::Input, tag(",")),
            value(Stat::Output, tag(".")),
            get_insert,
            get_while,
        )),
        get_whitespace,
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_assembly_inserts() {
        assert_eq!(
            parse("::ldr r3, =345::"),
            Ok(BrainFuck(vec![Stat::Asm("ldr r3, =345".to_string())]))
        );
        assert_eq!(
            parse("::ldr r3, =345\n@this is a comment\nmov r3, r4::"),
            Ok(BrainFuck(vec![Stat::Asm(
                "ldr r3, =345\n@this is a comment\nmov r3, r4".to_string()
            )]))
        );
        assert_eq!(
            parse("::::"),
            Ok(BrainFuck(vec![Stat::Asm("".to_string())]))
        );
        assert_eq!(
            parse("::::++"),
            Ok(BrainFuck(vec![
                Stat::Asm("".to_string()),
                Stat::DerefOp(Op::Add, 1),
                Stat::DerefOp(Op::Add, 1)
            ]))
        );
        assert_eq!(
            parse("--::::++"),
            Ok(BrainFuck(vec![
                Stat::DerefOp(Op::Add, -1),
                Stat::DerefOp(Op::Add, -1),
                Stat::Asm("".to_string()),
                Stat::DerefOp(Op::Add, 1),
                Stat::DerefOp(Op::Add, 1)
            ]))
        );
    }

    #[test]
    fn parses_while() {
        assert_eq!(
            parse(" \t\n [::ldr r3, =345::]"),
            Ok(BrainFuck(vec![Stat::WhileNonZero(vec![Stat::Asm(
                "ldr r3, =345".to_string()
            )])]))
        );
        assert_eq!(
            parse(" \t\n [::ldr r3, =345::++]++"),
            Ok(BrainFuck(vec![
                Stat::WhileNonZero(vec![
                    Stat::Asm("ldr r3, =345".to_string()),
                    Stat::DerefOp(Op::Add, 1),
                    Stat::DerefOp(Op::Add, 1)
                ]),
                Stat::DerefOp(Op::Add, 1),
                Stat::DerefOp(Op::Add, 1)
            ]))
        );
        assert_eq!(
            parse(" \t\n [[<]::ldr r3, =345::++]++"),
            Ok(BrainFuck(vec![
                Stat::WhileNonZero(vec![
                    Stat::WhileNonZero(vec![Stat::PtrMove(-1)]),
                    Stat::Asm("ldr r3, =345".to_string()),
                    Stat::DerefOp(Op::Add, 1),
                    Stat::DerefOp(Op::Add, 1)
                ]),
                Stat::DerefOp(Op::Add, 1),
                Stat::DerefOp(Op::Add, 1)
            ]))
        );
        assert_eq!(
            parse("[[[]]]"),
            Ok(BrainFuck(vec![Stat::WhileNonZero(vec![
                Stat::WhileNonZero(vec![Stat::WhileNonZero(vec![])])
            ])]))
        )
    }

    #[test]
    fn parse_skips_comments_and_whitespace() {
        // spaces, tabs, newlines
        assert_eq!(
            parse(" \t\n ::ldr r3, =345::"),
            Ok(BrainFuck(vec![Stat::Asm("ldr r3, =345".to_string())]))
        );
        assert_eq!(
            parse("::ldr r3, =345\n@this is a comment\nmov r3, r4:: \r\n"),
            Ok(BrainFuck(vec![Stat::Asm(
                "ldr r3, =345\n@this is a comment\nmov r3, r4".to_string()
            )]))
        );
        assert_eq!(
            parse(" \t:::: \r\n"),
            Ok(BrainFuck(vec![Stat::Asm("".to_string())]))
        );
        assert_eq!(
            parse(" \t\t:::: +\r\n + "),
            Ok(BrainFuck(vec![
                Stat::Asm("".to_string()),
                Stat::DerefOp(Op::Add, 1),
                Stat::DerefOp(Op::Add, 1)
            ]))
        );
        assert_eq!(
            parse("- - ::::+\n+ "),
            Ok(BrainFuck(vec![
                Stat::DerefOp(Op::Add, -1),
                Stat::DerefOp(Op::Add, -1),
                Stat::Asm("".to_string()),
                Stat::DerefOp(Op::Add, 1),
                Stat::DerefOp(Op::Add, 1)
            ]))
        );

        // comments
        assert_eq!(
            parse("# hello #::ldr r3, =345::"),
            Ok(BrainFuck(vec![Stat::Asm("ldr r3, =345".to_string())]))
        );
        assert_eq!(parse("::ldr r3, =345\n@this is a comment\nmov r3, r4::# hello, this is not an insert :: asm :: #"), Ok(BrainFuck(vec![Stat::Asm("ldr r3, =345\n@this is a comment\nmov r3, r4".to_string())])));
        assert_eq!(
            parse("##::::# hello #"),
            Ok(BrainFuck(vec![Stat::Asm("".to_string())]))
        );
        assert_eq!(
            parse("::::# hello ## world #+# hello #+# hello #"),
            Ok(BrainFuck(vec![
                Stat::Asm("".to_string()),
                Stat::DerefOp(Op::Add, 1),
                Stat::DerefOp(Op::Add, 1)
            ]))
        );
        assert_eq!(
            parse("#++++#--::::#<<-->>#++"),
            Ok(BrainFuck(vec![
                Stat::DerefOp(Op::Add, -1),
                Stat::DerefOp(Op::Add, -1),
                Stat::Asm("".to_string()),
                Stat::DerefOp(Op::Add, 1),
                Stat::DerefOp(Op::Add, 1)
            ]))
        );

        // both
        assert_eq!(
            parse("# hello # \t::ldr r3, =345::\n"),
            Ok(BrainFuck(vec![Stat::Asm("ldr r3, =345".to_string())]))
        );
        assert_eq!(parse(" \t \t::ldr r3, =345\n@this is a comment\nmov r3, r4::# hello, this is not an insert :: asm :: #\n"), Ok(BrainFuck(vec![Stat::Asm("ldr r3, =345\n@this is a comment\nmov r3, r4".to_string())])));
        assert_eq!(
            parse("##\r\n::::# hello #\t"),
            Ok(BrainFuck(vec![Stat::Asm("".to_string())]))
        );
        assert_eq!(
            parse("::::\t# hello # \n # world #+# hello #+  # hello #"),
            Ok(BrainFuck(vec![
                Stat::Asm("".to_string()),
                Stat::DerefOp(Op::Add, 1),
                Stat::DerefOp(Op::Add, 1)
            ]))
        );
        assert_eq!(
            parse("#++++#--::::#<<-->>#\n++"),
            Ok(BrainFuck(vec![
                Stat::DerefOp(Op::Add, -1),
                Stat::DerefOp(Op::Add, -1),
                Stat::Asm("".to_string()),
                Stat::DerefOp(Op::Add, 1),
                Stat::DerefOp(Op::Add, 1)
            ]))
        );
    }
}
