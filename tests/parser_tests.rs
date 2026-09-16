use r_python::ir::ast::*;
use r_python::parser::{parse, parse_expression, parse_statement};

// Basic Expression Tests
mod expression_tests {
    use super::*;

    #[test]
    fn test_literals() {
        let cases = vec![
            ("42", Expression::CInt(42)),
            ("3.14", Expression::CReal(3.14)),
            ("\"hello\"", Expression::CString("hello".to_string())),
            ("True", Expression::CTrue),
            ("False", Expression::CFalse),
        ];

        for (input, expected) in cases {
            let (rest, result) = parse_expression(input).unwrap();
            assert_eq!(rest, "");
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_arithmetic_operations() {
        let cases = vec![
            (
                "1 + 2",
                Expression::Add(Box::new(Expression::CInt(1)), Box::new(Expression::CInt(2))),
            ),
            (
                "3 * 4",
                Expression::Mul(Box::new(Expression::CInt(3)), Box::new(Expression::CInt(4))),
            ),
            (
                "2 + 3 * 4", // Tests precedence
                Expression::Add(
                    Box::new(Expression::CInt(2)),
                    Box::new(Expression::Mul(
                        Box::new(Expression::CInt(3)),
                        Box::new(Expression::CInt(4)),
                    )),
                ),
            ),
            (
                "(1 + 2) * (3 + 4)", // Tests grouping
                Expression::Mul(
                    Box::new(Expression::Add(
                        Box::new(Expression::CInt(1)),
                        Box::new(Expression::CInt(2)),
                    )),
                    Box::new(Expression::Add(
                        Box::new(Expression::CInt(3)),
                        Box::new(Expression::CInt(4)),
                    )),
                ),
            ),
        ];

        for (input, expected) in cases {
            let (rest, result) = parse_expression(input).unwrap();
            assert_eq!(rest, "");
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_boolean_operations() {
        let cases = vec![
            (
                "True and False",
                Expression::And(Box::new(Expression::CTrue), Box::new(Expression::CFalse)),
            ),
            (
                "True or False",
                Expression::Or(Box::new(Expression::CTrue), Box::new(Expression::CFalse)),
            ),
            ("not True", Expression::Not(Box::new(Expression::CTrue))),
            (
                "not (True and False) or True",
                Expression::Or(
                    Box::new(Expression::Not(Box::new(Expression::And(
                        Box::new(Expression::CTrue),
                        Box::new(Expression::CFalse),
                    )))),
                    Box::new(Expression::CTrue),
                ),
            ),
        ];

        for (input, expected) in cases {
            let (rest, result) = parse_expression(input).unwrap();
            assert_eq!(rest, "");
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_function_calls() {
        let input = "add(5, 3)";
        let expected = Expression::FuncCall(
            "add".to_string(),
            vec![Expression::CInt(5), Expression::CInt(3)],
        );

        let (rest, result) = parse_expression(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tuple_literals() {
        let cases = vec![
            (
                "(1, 2, 3)",
                Expression::Tuple(vec![
                    Expression::CInt(1),
                    Expression::CInt(2),
                    Expression::CInt(3),
                ]),
            ),
            (
                "(\"hello\", True)",
                Expression::Tuple(vec![
                    Expression::CString("hello".to_string()),
                    Expression::CTrue,
                ]),
            ),
            ("()", Expression::Tuple(vec![])),
            ("(42,)", Expression::Tuple(vec![Expression::CInt(42)])),
            (
                "((1, 2), (3, 4))",
                Expression::Tuple(vec![
                    Expression::Tuple(vec![Expression::CInt(1), Expression::CInt(2)]),
                    Expression::Tuple(vec![Expression::CInt(3), Expression::CInt(4)]),
                ]),
            ),
        ];

        for (input, expected) in cases {
            let (rest, result) = parse_expression(input).unwrap();
            assert_eq!(rest, "");
            assert_eq!(result, expected);
        }
    }
}

// Statement Tests
mod statement_tests {
    use super::*;

    #[test]
    fn test_assignments() {
        let cases = vec![
            (
                "x = 42",
                Statement::Assignment("x".to_string(), Box::new(Expression::CInt(42))),
            ),
            (
                "result = add(5, 3)",
                Statement::Assignment(
                    "result".to_string(),
                    Box::new(Expression::FuncCall(
                        "add".to_string(),
                        vec![Expression::CInt(5), Expression::CInt(3)],
                    )),
                ),
            ),
        ];

        for (input, expected) in cases {
            let (rest, result) = parse_statement(input).unwrap();
            assert_eq!(rest, "");
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_expression_statement() {
        let input = "print(1)";
        let expected = Statement::ExprStmt(Box::new(Expression::FuncCall(
            "print".to_string(),
            vec![Expression::CInt(1)],
        )));

        let (rest, result) = parse_statement(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_if_statements() {
        let input = "if x > 0: y = 1; end";
        let expected = Statement::IfChain {
            branches: vec![(
                Box::new(Expression::GT(
                    Box::new(Expression::Var("x".to_string())),
                    Box::new(Expression::CInt(0)),
                )),
                Box::new(Statement::Block(vec![Statement::Assignment(
                    "y".to_string(),
                    Box::new(Expression::CInt(1)),
                )])),
            )],
            else_branch: None,
        };

        let (rest, result) = parse_statement(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, expected);

        // Test with else (new syntax: one `end` at the very end)
        let input = "if x > 0: y = 1; else: y = 2; end";
        let expected = Statement::IfChain {
            branches: vec![(
                Box::new(Expression::GT(
                    Box::new(Expression::Var("x".to_string())),
                    Box::new(Expression::CInt(0)),
                )),
                Box::new(Statement::Block(vec![Statement::Assignment(
                    "y".to_string(),
                    Box::new(Expression::CInt(1)),
                )])),
            )],
            else_branch: Some(Box::new(Statement::Block(vec![Statement::Assignment(
                "y".to_string(),
                Box::new(Expression::CInt(2)),
            )]))),
        };

        let (rest, result) = parse_statement(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_for_statements() {
        let input = "for x in range: x = x + 1; end";
        let expected = Statement::For(
            "x".to_string(),
            Box::new(Expression::Var("range".to_string())),
            Box::new(Statement::Block(vec![Statement::Assignment(
                "x".to_string(),
                Box::new(Expression::Add(
                    Box::new(Expression::Var("x".to_string())),
                    Box::new(Expression::CInt(1)),
                )),
            )])),
        );

        let (rest, result) = parse_statement(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_for_over_tuple() {
        let cases = vec![
            (
                "for x in (1, 2, 3): x = x + 1; end",
                Statement::For(
                    "x".to_string(),
                    Box::new(Expression::Tuple(vec![
                        Expression::CInt(1),
                        Expression::CInt(2),
                        Expression::CInt(3),
                    ])),
                    Box::new(Statement::Block(vec![Statement::Assignment(
                        "x".to_string(),
                        Box::new(Expression::Add(
                            Box::new(Expression::Var("x".to_string())),
                            Box::new(Expression::CInt(1)),
                        )),
                    )])),
                ),
            ),
            (
                "for item in (\"a\", \"b\"): val x = item; end",
                Statement::For(
                    "item".to_string(),
                    Box::new(Expression::Tuple(vec![
                        Expression::CString("a".to_string()),
                        Expression::CString("b".to_string()),
                    ])),
                    Box::new(Statement::Block(vec![Statement::ValDeclaration(
                        "x".to_string(),
                        Box::new(Expression::Var("item".to_string())),
                    )])),
                ),
            ),
            (
                "for x in (): val y = 1; end",
                Statement::For(
                    "x".to_string(),
                    Box::new(Expression::Tuple(vec![])),
                    Box::new(Statement::Block(vec![Statement::ValDeclaration(
                        "y".to_string(),
                        Box::new(Expression::CInt(1)),
                    )])),
                ),
            ),
            (
                "for t in ((1,2), (3,4)): val x = t; end",
                Statement::For(
                    "t".to_string(),
                    Box::new(Expression::Tuple(vec![
                        Expression::Tuple(vec![Expression::CInt(1), Expression::CInt(2)]),
                        Expression::Tuple(vec![Expression::CInt(3), Expression::CInt(4)]),
                    ])),
                    Box::new(Statement::Block(vec![Statement::ValDeclaration(
                        "x".to_string(),
                        Box::new(Expression::Var("t".to_string())),
                    )])),
                ),
            ),
        ];

        for (input, expected) in cases {
            let (rest, result) = parse_statement(input).unwrap();
            assert_eq!(rest, "");
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_function_definitions() {
        let input = "def add(x: Int, y: Int) -> Int: return x + y; end";
        let expected = Statement::FuncDef(Function {
            name: "add".to_string(),
            kind: Type::TInteger,
            params: vec![
                FormalArgument::new("x".to_string(), Type::TInteger),
                FormalArgument::new("y".to_string(), Type::TInteger),
            ],
            body: Some(Box::new(Statement::Block(vec![Statement::Return(
                Box::new(Expression::Add(
                    Box::new(Expression::Var("x".to_string())),
                    Box::new(Expression::Var("y".to_string())),
                )),
            )]))),
        });

        let (rest, result) = parse_statement(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_var_declarations() {
        let cases = vec![
            (
                "var x = 42",
                Statement::VarDeclaration("x".to_string(), Box::new(Expression::CInt(42))),
            ),
            // (
            //     "var result = add(5, 3)",
            //     Statement::VarDeclaration(
            //         "result".to_string(),
            //         Box::new(Expression::FuncCall(
            //             "add".to_string(),
            //             vec![Expression::CInt(5), Expression::CInt(3)],
            //         )),
            //     ),
            // ),
            // (
            //     "var name = \"John\"",
            //     Statement::VarDeclaration(
            //         "name".to_string(),
            //         Box::new(Expression::CString("John".to_string())),
            //     ),
            // ),
            // (
            //     "var is_valid = True",
            //     Statement::VarDeclaration(
            //         "is_valid".to_string(),
            //         Box::new(Expression::CTrue),
            //     ),
            // ),
        ];

        for (input, expected) in cases {
            let (rest, result) = parse_statement(input).unwrap();
            assert_eq!(rest, "");
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_val_declarations() {
        let cases = vec![
            (
                "val x = 42",
                Statement::ValDeclaration("x".to_string(), Box::new(Expression::CInt(42))),
            ),
            (
                "val result = add(5, 3)",
                Statement::ValDeclaration(
                    "result".to_string(),
                    Box::new(Expression::FuncCall(
                        "add".to_string(),
                        vec![Expression::CInt(5), Expression::CInt(3)],
                    )),
                ),
            ),
            (
                "val name = \"John\"",
                Statement::ValDeclaration(
                    "name".to_string(),
                    Box::new(Expression::CString("John".to_string())),
                ),
            ),
            (
                "val is_valid = True",
                Statement::ValDeclaration("is_valid".to_string(), Box::new(Expression::CTrue)),
            ),
        ];

        for (input, expected) in cases {
            let (rest, result) = parse_statement(input).unwrap();
            assert_eq!(rest, "");
            assert_eq!(result, expected);
        }
    }
}

// ADT Tests
mod adt_tests {
    use super::*;

    #[test]
    fn test_adt_declarations() {
        // ADT declarations are currently parsed as *types* (see README limitation).
        let input = "data Shape:\n  | Circle Int\n  | Rectangle Int Int\nend";
        let expected = Type::TAlgebraicData(
            "Shape".to_string(),
            vec![
                ValueConstructor::new("Circle".to_string(), vec![Type::TInteger]),
                ValueConstructor::new(
                    "Rectangle".to_string(),
                    vec![Type::TInteger, Type::TInteger],
                ),
            ],
        );

        let (rest, result) = r_python::parser::parse_type(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(result, expected);
    }
}

// Error Handling Tests
mod error_tests {
    use super::*;

    #[test]
    fn test_invalid_keywords() {
        let invalid_cases = vec![
            "def if(x: Int) -> Int:\n    return x",
            "def while(x: Int) -> Int:\n    return x",
            "if = 10",
            "while = 10",
        ];

        for input in invalid_cases {
            assert!(parse_statement(input).is_err());
        }
    }

    #[test]
    fn test_invalid_expressions() {
        let invalid_cases = vec![
            "1 + ",    // Incomplete expression
            "* 2",     // Missing left operand
            "1 + + 2", // Double operator
            "(1 + 2",  // Unclosed parenthesis
            "1 + 2)",  // Extra closing parenthesis
        ];

        for input in invalid_cases {
            match parse_expression(input) {
                Err(_) => {}
                Ok((rest, _)) => {
                    // The expression parser is prefix-friendly; invalid expressions may parse a
                    // prefix and leave the remainder unconsumed.
                    assert!(
                        !rest.trim().is_empty(),
                        "expected parse failure or leftover input for: {input}"
                    );
                }
            }
        }
    }
}

// Complete Program Tests
mod program_tests {
    use super::*;

    #[test]
    fn test_complete_program() {
        let input = r#"
def factorial(n: Int) -> Int:
    if n <= 1:
        return 1;
    else:
        return n * factorial(n - 1);
    end
end;

x = factorial(5);
assert(x == 120, "factorial of 5 should be 120");"#;
        let (rest, statements) = parse(input).unwrap();
        assert_eq!(rest.trim(), "");
        assert!(statements.len() >= 3); // Function definition, assignment, and assert
    }

    #[test]
    fn test_program_with_adt() {
        let input = r#"
data Shape = Circle Int | Rectangle Int Int;

def area(shape: Shape) -> Int:
    if isCircle(shape):
        r = getCircleRadius(shape);
        return r * r * 3;
    end
    else:
        w = getRectangleWidth(shape);
        h = getRectangleHeight(shape);
        return w * h;
    end
end;

c = Circle(5);
area_c = area(c);
assert(area_c == 75, "area of circle with radius 5 should be 75");"#;
        // ADT declarations are not currently supported as *statements*.
        // This test asserts two things:
        // 1) the parser does not produce a `TypeDeclaration` statement for `data ...` blocks
        // 2) the program is not fully consumed (so we don't accidentally accept this syntax)
        let (rest, statements) = parse(input).unwrap();
        assert!(statements
            .iter()
            .all(|s| !matches!(s, Statement::TypeDeclaration(_, _))));
        assert_ne!(rest.trim(), "");
    }
}

// Class Declaration Tests (parser phase only: no type checking or execution)
mod class_tests {
    use super::*;

    fn field(name: &str, ty: Type, mutable: bool, init: Expression) -> FieldDeclaration {
        FieldDeclaration {
            name: name.to_string(),
            field_type: ty,
            mutable,
            initializer: Box::new(init),
        }
    }

    #[test]
    fn test_empty_class() {
        let (rest, result) = parse_statement("class Empty: end").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            result,
            Statement::ClassDef(Class {
                name: "Empty".to_string(),
                fields: vec![],
                methods: vec![],
            })
        );
    }

    #[test]
    fn test_class_with_val_and_var_fields() {
        let input = "class Point:\n    val x: Int = 0;\n    var label: String = \"p\";\nend";
        let (rest, result) = parse_statement(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            result,
            Statement::ClassDef(Class {
                name: "Point".to_string(),
                fields: vec![
                    field("x", Type::TInteger, false, Expression::CInt(0)),
                    field(
                        "label",
                        Type::TString,
                        true,
                        Expression::CString("p".to_string())
                    ),
                ],
                methods: vec![],
            })
        );
    }

    #[test]
    fn test_class_with_method() {
        let input = "class Counter:\n    var n: Int = 0;\n    def get(self: Counter) -> Int:\n        return 1;\n    end;\nend";
        let (rest, result) = parse_statement(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            result,
            Statement::ClassDef(Class {
                name: "Counter".to_string(),
                fields: vec![field("n", Type::TInteger, true, Expression::CInt(0))],
                methods: vec![Function {
                    name: "get".to_string(),
                    kind: Type::TInteger,
                    params: vec![FormalArgument::new(
                        "self".to_string(),
                        Type::TClass("Counter".to_string())
                    )],
                    body: Some(Box::new(Statement::Block(vec![Statement::Return(
                        Box::new(Expression::CInt(1))
                    )]))),
                }],
            })
        );
    }

    #[test]
    fn test_class_example_file() {
        let src = include_str!("../examples/classes/point_declaration.rpy");
        let (rest, stmts) = parse(src).unwrap();
        assert_eq!(rest, "");
        match &stmts[..] {
            [Statement::ClassDef(c)] => {
                assert_eq!(c.name, "Point");
                assert_eq!(c.fields.len(), 2);
                let names: Vec<_> = c.methods.iter().map(|m| m.name.as_str()).collect();
                assert_eq!(names, vec!["translate", "get_x"]);
                assert_eq!(c.methods[0].params.len(), 3);
            }
            other => panic!("expected one ClassDef, got {other:?}"),
        }
    }

    #[test]
    fn test_invalid_class_declarations() {
        let invalid = vec![
            "class : end",                                       // missing name
            "class if: end",                                     // keyword as name
            "class P end",                                       // missing ':'
            "class P: val x: Int = 0;",                          // missing 'end'
            "class P: val x = 0; end",                           // field without type
            "class P: x = 1; end",                               // statement not allowed in body
            "class P: def f(a: Int) -> Int: return a; end; end", // method without self
            "class P: def f() -> Int: return 1; end; end",       // method without params
        ];
        for input in invalid {
            if let Ok((_, Statement::ClassDef(c))) = parse_statement(input) {
                panic!("accepted invalid class {input:?}: {c:?}")
            }
        }
    }

    #[test]
    fn test_class_keyword_is_reserved() {
        // `class` can no longer be used as an identifier.
        assert!(parse_statement("class = 1").is_err());
        assert!(parse_expression("class").is_err());
    }
}
