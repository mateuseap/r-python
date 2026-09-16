use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, multispace0},
    combinator::{map, recognize},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::{preceded, tuple},
    IResult,
};

use crate::ir::ast::{Type, ValueConstructor};

use crate::parser::parser_common::{
    identifier, keyword, separator, ANY_TYPE, BOOLEAN_TYPE, COLON_CHAR, COMMA_CHAR, COMMA_SYMBOL,
    DATA_KEYWORD, END_KEYWORD, FUNCTION_ARROW, INT_TYPE, LEFT_BRACKET, LEFT_PAREN, MAYBE_TYPE,
    PIPE_CHAR, REAL_TYPE, RESULT_TYPE, RIGHT_BRACKET, RIGHT_PAREN, STRING_TYPE, UNIT_TYPE,
};

pub fn parse_type(input: &str) -> IResult<&str, Type> {
    alt((
        parse_basic_types,
        parse_list_type,
        parse_tuple_type,
        parse_maybe_type,
        parse_result_type,
        parse_function_type,
        parse_adt_type,
        parse_class_type,
    ))(input)
}

/// A bare identifier in type position names a class, e.g. `self: Point`.
/// Tried last so built-in type names always win.
fn parse_class_type(input: &str) -> IResult<&str, Type> {
    map(preceded(multispace0, identifier), |name| {
        Type::TClass(name.to_string())
    })(input)
}

fn parse_basic_types(input: &str) -> IResult<&str, Type> {
    map(
        alt((
            keyword(INT_TYPE),
            keyword(REAL_TYPE),
            keyword(BOOLEAN_TYPE),
            keyword(STRING_TYPE),
            keyword(UNIT_TYPE),
            keyword(ANY_TYPE),
        )),
        |t| match t {
            INT_TYPE => Type::TInteger,
            REAL_TYPE => Type::TReal,
            BOOLEAN_TYPE => Type::TBool,
            STRING_TYPE => Type::TString,
            UNIT_TYPE => Type::TVoid,
            ANY_TYPE => Type::TAny,
            _ => unreachable!(),
        },
    )(input)
}

fn parse_list_type(input: &str) -> IResult<&str, Type> {
    map(
        tuple((
            preceded(multispace0, keyword("List")),
            preceded(multispace0, char(LEFT_BRACKET)),
            preceded(multispace0, parse_type),
            preceded(multispace0, char(RIGHT_BRACKET)),
        )),
        |(_, _, t, _)| Type::TList(Box::new(t)),
    )(input)
}

fn parse_tuple_type(input: &str) -> IResult<&str, Type> {
    map(
        tuple((
            preceded(multispace0, keyword("Tuple")),
            preceded(multispace0, char(LEFT_BRACKET)),
            preceded(
                multispace0,
                separated_list1(separator(COMMA_SYMBOL), parse_type),
            ),
            preceded(multispace0, char(RIGHT_BRACKET)),
        )),
        |(_, _, ts, _)| Type::TTuple(ts),
    )(input)
}

fn parse_maybe_type(input: &str) -> IResult<&str, Type> {
    map(
        tuple((
            preceded(multispace0, keyword(MAYBE_TYPE)),
            preceded(multispace0, char(LEFT_BRACKET)),
            preceded(multispace0, parse_type),
            preceded(multispace0, char(RIGHT_BRACKET)),
        )),
        |(_, _, t, _)| Type::TMaybe(Box::new(t)),
    )(input)
}

fn parse_result_type(input: &str) -> IResult<&str, Type> {
    map(
        tuple((
            preceded(multispace0, keyword(RESULT_TYPE)),
            preceded(multispace0, char(LEFT_BRACKET)),
            preceded(multispace0, parse_type),
            preceded(multispace0, char(COMMA_CHAR)),
            preceded(multispace0, parse_type),
            preceded(multispace0, char(RIGHT_BRACKET)),
        )),
        |(_, _, t_ok, _, t_err, _)| Type::TResult(Box::new(t_ok), Box::new(t_err)),
    )(input)
}

pub fn parse_function_type(input: &str) -> IResult<&str, Type> {
    map(
        tuple((
            preceded(multispace0, keyword("fn")),
            preceded(multispace0, char(LEFT_PAREN)),
            preceded(
                multispace0,
                separated_list0(separator(COMMA_SYMBOL), parse_type),
            ),
            preceded(multispace0, char(RIGHT_PAREN)),
            preceded(multispace0, tag(FUNCTION_ARROW)),
            preceded(multispace0, parse_type),
        )),
        |(_, _, t_args, _, _, t_ret)| Type::TFunction(Box::new(t_ret), t_args),
    )(input)
}

fn parse_adt_type(input: &str) -> IResult<&str, Type> {
    map(
        tuple((
            keyword(DATA_KEYWORD),
            preceded(multispace0, identifier),
            preceded(multispace0, char(COLON_CHAR)),
            many1(parse_adt_cons),
            preceded(multispace0, keyword(END_KEYWORD)),
        )),
        |(_, name, _, cons, _)| Type::TAlgebraicData(name.to_string(), cons),
    )(input)
}

fn parse_adt_cons(input: &str) -> IResult<&str, ValueConstructor> {
    // Constructor syntax: `| Name` or `| Name Type`
    let (input, _) = preceded(multispace0, char(PIPE_CHAR))(input)?;
    let (input, _) = multispace0(input)?;
    // Use constructor_name instead of identifier to allow keywords like Just/Nothing
    let (input, name) = constructor_name(input)?;
    // Use parse_basic_types to avoid infinite recursion through parse_type -> parse_adt_type.
    // Allow zero or more field types (e.g., `| Rectangle Int Int`).
    let (input, types) = many0(preceded(multispace0, parse_basic_types))(input)?;

    Ok((input, ValueConstructor::new(name.to_string(), types)))
}

/// Parses a constructor name (allows keywords like Just, Nothing, Ok, Err)
fn constructor_name(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        alt((alpha1, tag("_"))),
        many0(alt((alpha1, tag("_"), digit1))),
    )))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_types() {
        assert_eq!(parse_basic_types("Int"), Ok(("", Type::TInteger)));
        assert_eq!(parse_basic_types("Boolean"), Ok(("", Type::TBool)));
    }

    #[test]
    fn test_parse_list_type() {
        assert_eq!(
            parse_list_type("List[Int]"),
            Ok(("", Type::TList(Box::new(Type::TInteger))))
        );
    }

    #[test]
    fn test_parse_tuple_type() {
        assert_eq!(
            parse_tuple_type("Tuple[Int, Real]"),
            Ok(("", Type::TTuple(vec![Type::TInteger, Type::TReal])))
        );
    }

    #[test]
    fn test_parse_maybe_type() {
        assert_eq!(
            parse_maybe_type("Maybe [Boolean]"),
            Ok(("", Type::TMaybe(Box::new(Type::TBool))))
        );
    }

    #[test]
    fn test_parse_result_type() {
        assert_eq!(
            parse_result_type("Result [Int, String]"),
            Ok((
                "",
                Type::TResult(Box::new(Type::TInteger), Box::new(Type::TString))
            ))
        );
    }

    #[test]
    fn test_parse_function_type() {
        assert_eq!(
            parse_function_type("fn(Int, Boolean) -> String"),
            Ok((
                "",
                Type::TFunction(Box::new(Type::TString), vec![Type::TInteger, Type::TBool])
            ))
        );
    }

    #[test]
    fn test_parse_adt_type() {
        let input = "data Maybe:\n  | Just Int\n  | Nothing\nend";
        let expected = Type::TAlgebraicData(
            "Maybe".to_string(),
            vec![
                ValueConstructor::new("Just".to_string(), vec![Type::TInteger]),
                ValueConstructor::new("Nothing".to_string(), vec![]),
            ],
        );
        assert_eq!(parse_adt_type(input), Ok(("", expected)));
    }
}
