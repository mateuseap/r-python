use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{char, digit1, multispace0},
    combinator::{map, map_res, opt, value},
    error::Error,
    multi::{fold_many0, separated_list0},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

use std::str::FromStr;

use crate::ir::ast::Function;
use crate::ir::ast::Statement;
use crate::parser::parser_common::{
    identifier,
    is_string_char,
    keyword,
    COLON_CHAR,
    // Other character constants
    COMMA_CHAR,
    FUNCTION_ARROW,
    LAMBDA_KEYWORD,
    // Bracket and parentheses constants
    LEFT_BRACKET,
    LEFT_PAREN,
    RIGHT_BRACKET,
    RIGHT_PAREN,
};
use crate::parser::parser_stmt::{parse_formal_argument, parse_return_statement};
use crate::parser::parser_type::parse_type;
use crate::{ir::ast::Expression, parser::parser_common::END_KEYWORD};

pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
    parse_or(input)
}

fn parse_or(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_and(input)?;
    fold_many0(
        preceded(keyword("or"), parse_and),
        move || init.clone(),
        |acc, val| Expression::Or(Box::new(acc), Box::new(val)),
    )(input)
}

fn parse_and(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_not(input)?;
    fold_many0(
        preceded(keyword("and"), parse_not),
        move || init.clone(),
        |acc, val| Expression::And(Box::new(acc), Box::new(val)),
    )(input)
}

fn parse_not(input: &str) -> IResult<&str, Expression> {
    alt((
        map(preceded(keyword("not"), parse_not), |e| {
            Expression::Not(Box::new(e))
        }),
        parse_relational,
    ))(input)
}

fn parse_relational(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_add_sub(input)?;
    fold_many0(
        pair(
            alt((
                operator("<="),
                operator("<"),
                operator(">="),
                operator(">"),
                operator("=="),
                operator("!="),
            )),
            parse_add_sub,
        ),
        move || init.clone(),
        |acc, (op, val)| match op {
            "<" => Expression::LT(Box::new(acc), Box::new(val)),
            "<=" => Expression::LTE(Box::new(acc), Box::new(val)),
            ">" => Expression::GT(Box::new(acc), Box::new(val)),
            ">=" => Expression::GTE(Box::new(acc), Box::new(val)),
            "==" => Expression::EQ(Box::new(acc), Box::new(val)),
            "!=" => Expression::NEQ(Box::new(acc), Box::new(val)),
            _ => unreachable!(),
        },
    )(input)
}

fn parse_add_sub(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_term(input)?;
    fold_many0(
        pair(alt((operator("+"), operator("-"))), parse_term),
        move || init.clone(),
        |acc, (op, val)| match op {
            "+" => Expression::Add(Box::new(acc), Box::new(val)),
            "-" => Expression::Sub(Box::new(acc), Box::new(val)),
            _ => unreachable!(),
        },
    )(input)
}

fn parse_term(input: &str) -> IResult<&str, Expression> {
    let (input, init) = parse_factor(input)?;
    fold_many0(
        pair(alt((operator("*"), operator("/"))), parse_factor),
        move || init.clone(),
        |acc, (op, val)| match op {
            "*" => Expression::Mul(Box::new(acc), Box::new(val)),
            "/" => Expression::Div(Box::new(acc), Box::new(val)),
            _ => unreachable!(),
        },
    )(input)
}

fn parse_factor(input: &str) -> IResult<&str, Expression> {
    alt((
        parse_bool,
        parse_maybe_result_constructors,
        parse_error_handling_operations,
        parse_number,
        parse_string,
        parse_list,
        parse_function_call,
        parse_var,
        parse_paren_or_tuple,
        parse_lambda,
    ))(input)
}

fn parse_maybe_result_constructors(input: &str) -> IResult<&str, Expression> {
    alt((
        value(Expression::CNothing, keyword("Nothing")),
        map(
            tuple((
                keyword("Just"),
                multispace0,
                char::<&str, Error<&str>>(LEFT_PAREN),
                multispace0,
                parse_expression,
                multispace0,
                char::<&str, Error<&str>>(RIGHT_PAREN),
            )),
            |(_, _, _, _, expr, _, _)| Expression::CJust(Box::new(expr)),
        ),
        map(
            tuple((
                keyword("Ok"),
                multispace0,
                char::<&str, Error<&str>>(LEFT_PAREN),
                multispace0,
                parse_expression,
                multispace0,
                char::<&str, Error<&str>>(RIGHT_PAREN),
            )),
            |(_, _, _, _, expr, _, _)| Expression::COk(Box::new(expr)),
        ),
        map(
            tuple((
                keyword("Err"),
                multispace0,
                char::<&str, Error<&str>>(LEFT_PAREN),
                multispace0,
                parse_expression,
                multispace0,
                char::<&str, Error<&str>>(RIGHT_PAREN),
            )),
            |(_, _, _, _, expr, _, _)| Expression::CErr(Box::new(expr)),
        ),
    ))(input)
}

fn parse_error_handling_operations(input: &str) -> IResult<&str, Expression> {
    alt((
        map(
            tuple((
                keyword("unwrap"),
                multispace0,
                char::<&str, Error<&str>>(LEFT_PAREN),
                multispace0,
                parse_expression,
                multispace0,
                char::<&str, Error<&str>>(RIGHT_PAREN),
            )),
            |(_, _, _, _, expr, _, _)| Expression::Unwrap(Box::new(expr)),
        ),
        map(
            tuple((
                keyword("tryUnwrap"),
                multispace0,
                char::<&str, Error<&str>>(LEFT_PAREN),
                multispace0,
                parse_expression,
                multispace0,
                char::<&str, Error<&str>>(RIGHT_PAREN),
            )),
            |(_, _, _, _, expr, _, _)| Expression::Propagate(Box::new(expr)),
        ),
        map(
            tuple((
                keyword("isNothing"),
                multispace0,
                char::<&str, Error<&str>>(LEFT_PAREN),
                multispace0,
                parse_expression,
                multispace0,
                char::<&str, Error<&str>>(RIGHT_PAREN),
            )),
            |(_, _, _, _, expr, _, _)| Expression::IsNothing(Box::new(expr)),
        ),
        map(
            tuple((
                keyword("isError"),
                multispace0,
                char::<&str, Error<&str>>(LEFT_PAREN),
                multispace0,
                parse_expression,
                multispace0,
                char::<&str, Error<&str>>(RIGHT_PAREN),
            )),
            |(_, _, _, _, expr, _, _)| Expression::IsError(Box::new(expr)),
        ),
    ))(input)
}

// Parses either a parenthesized expression or a tuple literal.
// Cases:
// - ()            => Tuple([])
// - (x,)          => Tuple([x])
// - (x, y, z)     => Tuple([...])
// - (expr)        => expr (grouping)
fn parse_paren_or_tuple(input: &str) -> IResult<&str, Expression> {
    let (input, _) = char::<&str, Error<&str>>(LEFT_PAREN)(input)?;
    let (input, _) = multispace0(input)?;

    // Try to parse an optional first expression
    let (input, first_opt) = opt(parse_expression)(input)?;
    let (input, _) = multispace0(input)?;

    // Try to consume a comma to decide if it's a tuple
    let (input_after_comma_try, comma_opt) = opt(delimited(
        multispace0,
        char::<&str, Error<&str>>(COMMA_CHAR),
        multispace0,
    ))(input)?;

    let (input, result_expr) = match (first_opt, comma_opt) {
        (None, _) => {
            // No expression inside: must be () => empty tuple
            (input, Expression::Tuple(vec![]))
        }
        (Some(first), Some(_)) => {
            // There was a comma: it's a tuple. Parse remaining elements (possibly zero)
            let (input, rest) = separated_list0(
                tuple((
                    multispace0,
                    char::<&str, Error<&str>>(COMMA_CHAR),
                    multispace0,
                )),
                parse_expression,
            )(input_after_comma_try)?;
            let mut elements = Vec::with_capacity(1 + rest.len());
            elements.push(first);
            elements.extend(rest);
            (input, Expression::Tuple(elements))
        }
        (Some(expr), None) => {
            // Single expression and no comma: grouping
            (input, expr)
        }
    };

    let (input, _) = multispace0(input)?;
    let (input, _) = char::<&str, Error<&str>>(RIGHT_PAREN)(input)?;
    Ok((input, result_expr))
}

pub fn parse_lambda(input: &str) -> IResult<&str, Expression> {
    map(
        tuple((
            preceded(keyword(LAMBDA_KEYWORD), multispace0),
            delimited(
                char::<&str, Error<&str>>(LEFT_PAREN),
                separated_list0(
                    tuple((
                        multispace0,
                        char::<&str, Error<&str>>(COMMA_CHAR),
                        multispace0,
                    )),
                    terminated(parse_formal_argument, multispace0),
                ),
                char::<&str, Error<&str>>(RIGHT_PAREN),
            ),
            preceded(multispace0, tag(FUNCTION_ARROW)),
            delimited(
                multispace0,
                parse_type,
                char::<&str, Error<&str>>(COLON_CHAR),
            ),
            delimited(multispace0, parse_return_statement, keyword(END_KEYWORD)),
        )),
        |(_, args, _, t, return_stmt)| {
            Expression::Lambda(Function {
                name: "".to_string(),
                kind: t,
                params: args,
                body: Some(Box::new(Statement::Block(vec![return_stmt]))),
            })
        },
    )(input)
}

fn parse_bool(input: &str) -> IResult<&str, Expression> {
    alt((
        value(Expression::CTrue, keyword("True")),
        value(Expression::CFalse, keyword("False")),
    ))(input)
}

fn parse_number(input: &str) -> IResult<&str, Expression> {
    let float_parser = map_res(
        tuple((
            opt(char::<&str, Error<&str>>('-')),
            digit1,
            char::<&str, Error<&str>>('.'),
            digit1,
        )),
        |(sign, d1, _, d2)| {
            let s = match sign {
                Some(_) => format!("-{}.{}", d1, d2),
                None => format!("{}.{}", d1, d2),
            };
            f64::from_str(&s)
        },
    );

    let int_parser = map_res(
        tuple((opt(char::<&str, Error<&str>>('-')), digit1)),
        |(sign, digits)| {
            let s = match sign {
                Some(_) => format!("-{}", digits),
                None => digits.to_string(),
            };
            i32::from_str(&s)
        },
    );

    alt((
        map(float_parser, Expression::CReal),
        map(int_parser, Expression::CInt),
    ))(input)
}

fn parse_string(input: &str) -> IResult<&str, Expression> {
    map(
        delimited(
            multispace0,
            delimited(
                char::<&str, Error<&str>>('"'),
                map(take_while(is_string_char), |s: &str| s.to_string()),
                char::<&str, Error<&str>>('"'),
            ),
            multispace0,
        ),
        |s| Expression::CString(s),
    )(input)
}

fn parse_var(input: &str) -> IResult<&str, Expression> {
    map(identifier, |v| Expression::Var(v.into()))(input)
}

fn parse_function_call(input: &str) -> IResult<&str, Expression> {
    let (input, name) = identifier(input)?;
    let (input, args) = parse_actual_arguments(input)?;
    Ok((input, Expression::FuncCall(name.to_string(), args)))
}

pub fn parse_actual_arguments(input: &str) -> IResult<&str, Vec<Expression>> {
    map(
        tuple((
            multispace0,
            char::<&str, Error<&str>>(LEFT_PAREN),
            separated_list0(
                tuple((
                    multispace0,
                    char::<&str, Error<&str>>(COMMA_CHAR),
                    multispace0,
                )),
                parse_expression,
            ),
            multispace0,
            char::<&str, Error<&str>>(RIGHT_PAREN),
        )),
        |(_, _, args, _, _)| args,
    )(input)
}

fn parse_list(input: &str) -> IResult<&str, Expression> {
    let (input, _) = multispace0(input)?;
    let (input, _) = char(LEFT_BRACKET)(input)?;
    let (input, _) = multispace0(input)?;

    let (input, elements) = separated_list0(
        delimited(multispace0, char(COMMA_CHAR), multispace0),
        parse_expression,
    )(input)?;

    let (input, _) = multispace0(input)?;
    let (input, _) = char(RIGHT_BRACKET)(input)?;
    let (input, _) = multispace0(input)?;

    Ok((input, Expression::ListValue(elements)))
}

/// Parses an operator.
fn operator<'a>(op: &'static str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    delimited(multispace0, tag(op), multispace0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expression_integer() {
        assert_eq!(
            parse_expression("123abc"),
            Ok(("abc", Expression::CInt(123)))
        );

        assert_eq!(
            parse_expression("-456 rest"),
            Ok((" rest", Expression::CInt(-456)))
        );
    }

    #[test]
    fn test_parse_expression_real() {
        assert_eq!(
            parse_expression("3.14xyz"),
            Ok(("xyz", Expression::CReal(3.14)))
        );

        assert_eq!(
            parse_expression("-0.001rest"),
            Ok(("rest", Expression::CReal(-0.001)))
        );
    }

    #[test]
    fn test_parse_expression_errors() {
        // Empty input should fail
        assert!(parse_expression("").is_err());

        // Tokens that cannot start an expression should fail
        assert!(parse_expression(")").is_err());
        assert!(parse_expression(",").is_err());
        assert!(parse_expression("]").is_err());

        // Ambiguous/invalid number forms may parse a prefix and leave remainder.
        match parse_expression("12.34.56") {
            Err(_) => {}
            Ok((rest, _)) => assert!(
                !rest.trim().is_empty(),
                "expected leftover input for invalid numeric form"
            ),
        }
    }

    #[test]
    fn test_keywords() {
        let cases = [
            ("if", "if"),
            ("else", "else"),
            ("while", "while"),
            ("and", "and"),
            ("or", "or"),
            ("not", "not"),
            ("for", "for"),
            ("def", "def"),
        ];

        for (input, expected) in cases {
            let result = keyword(expected)(input);
            assert!(result.is_ok(), "Failed to parse keyword '{}'", expected);
            let (rest, parsed) = result.unwrap();
            assert_eq!(parsed, expected);
            assert!(rest.is_empty() || rest.starts_with(' '));
        }
    }

    #[test]
    fn test_keyword_should_not_match_prefix_of_identifiers() {
        let mut parser = keyword("if");
        assert!(parser("iffy").is_err());

        let mut parser = keyword("def");
        assert!(parser("default").is_err());

        let mut parser = keyword("or");
        assert!(parser("origin").is_err());
    }

    #[test]
    fn test_parse_empty_list() {
        let input = "[]";
        let (rest, result) = parse_list(input).unwrap();
        assert_eq!(rest, "");
        assert!(matches!(result, Expression::ListValue(elements) if elements.is_empty()));
    }

    #[test]
    fn test_parse_integer_list() {
        let input = "[1, 2, 3]";
        let (rest, result) = parse_list(input).unwrap();
        assert_eq!(rest, "");
        if let Expression::ListValue(elements) = result {
            assert_eq!(elements.len(), 3);
            assert!(matches!(elements[0], Expression::CInt(1)));
            assert!(matches!(elements[1], Expression::CInt(2)));
            assert!(matches!(elements[2], Expression::CInt(3)));
        } else {
            panic!("Expected ListValue expression");
        }
    }

    #[test]
    fn test_parse_string_list() {
        let input = "[\"abc\", \"def\"]";
        let (rest, result) = parse_list(input).unwrap();
        assert_eq!(rest, "");
        if let Expression::ListValue(elements) = result {
            assert_eq!(elements.len(), 2);
            assert!(matches!(elements[0], Expression::CString(ref s) if s == "abc"));
            assert!(matches!(elements[1], Expression::CString(ref s) if s == "def"));
        } else {
            panic!("Expected ListValue expression");
        }
    }

    #[test]
    fn test_parse_list_with_spaces() {
        let input = "[ 1 , 2 , 3 ]";
        let (rest, result) = parse_list(input).unwrap();
        assert_eq!(rest, "");
        if let Expression::ListValue(elements) = result {
            assert_eq!(elements.len(), 3);
            assert!(matches!(elements[0], Expression::CInt(1)));
            assert!(matches!(elements[1], Expression::CInt(2)));
            assert!(matches!(elements[2], Expression::CInt(3)));
        } else {
            panic!("Expected ListValue expression");
        }
    }

    #[test]
    fn test_parse_list_with_outer_spaces() {
        let input = "    [1, 2, 3]    rest";
        let (rest, result) = parse_list(input).unwrap();
        assert_eq!(rest, "rest");
        if let Expression::ListValue(elements) = result {
            assert_eq!(elements.len(), 3);
            assert!(matches!(elements[0], Expression::CInt(1)));
            assert!(matches!(elements[1], Expression::CInt(2)));
            assert!(matches!(elements[2], Expression::CInt(3)));
        } else {
            panic!("Expected ListValue expression");
        }
    }

    #[test]
    fn test_parse_nested_list() {
        let input = "[[1, 2], [3, 4]]";
        let (rest, result) = parse_list(input).unwrap();
        assert_eq!(rest, "");
        if let Expression::ListValue(elements) = result {
            assert_eq!(elements.len(), 2);
            for element in elements {
                if let Expression::ListValue(inner) = element {
                    assert_eq!(inner.len(), 2);
                } else {
                    panic!("Expected inner ListValue expression");
                }
            }
        } else {
            panic!("Expected ListValue expression");
        }
    }

    #[test]
    fn test_parse_nothing_constructor() {
        let input = "Nothing";
        let result = parse_expression(input);
        assert_eq!(result, Ok(("", Expression::CNothing)));
    }

    #[test]
    fn test_parse_just_constructor() {
        let input = "Just(1)";
        let result = parse_expression(input);
        assert_eq!(
            result,
            Ok(("", Expression::CJust(Box::new(Expression::CInt(1)))))
        );
    }

    #[test]
    fn test_parse_ok_constructor() {
        let input = "Ok(\"ok\")";
        let result = parse_expression(input);
        assert_eq!(
            result,
            Ok((
                "",
                Expression::COk(Box::new(Expression::CString("ok".to_string())))
            ))
        );
    }

    #[test]
    fn test_parse_err_constructor() {
        let input = "Err(\"bad\")";
        let result = parse_expression(input);
        assert_eq!(
            result,
            Ok((
                "",
                Expression::CErr(Box::new(Expression::CString("bad".to_string())))
            ))
        );
    }
}

#[cfg(test)]
mod lambda_tests {
    use super::*;

    #[test]
    fn test_parse_lambda_simple() {
        let input = "lambda (x: Int) -> Int: return x end";
        let result = parse_lambda(input);
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
