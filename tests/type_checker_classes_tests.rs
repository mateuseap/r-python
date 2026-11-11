// Type checker tests for classes (phase 2).
// Programs are written as RPython source and parsed, so each test reads like the language.

use r_python::environment::environment::Environment;
use r_python::ir::ast::Type;
use r_python::parser::{parse, parse_expression};
use r_python::type_checker::{check_expr, check_stmt};

/// Type checks a whole program and returns the final environment.
fn check(src: &str) -> Result<Environment<Type>, String> {
    let (rest, stmts) = parse(src).expect("program should parse");
    assert_eq!(rest.trim(), "", "program was not fully parsed");
    let mut env = Environment::new();
    for stmt in stmts {
        env = check_stmt(stmt, &env)?;
    }
    Ok(env)
}

/// Type checks `decls`, then returns the type of `expr` in that environment.
fn type_of(decls: &str, expr: &str) -> Result<Type, String> {
    let env = check(decls).expect("declarations should type check");
    let (rest, e) = parse_expression(expr).expect("expression should parse");
    assert_eq!(rest.trim(), "", "expression was not fully parsed");
    check_expr(e, &env)
}

fn assert_error_mentions<T>(result: Result<T, String>, needle: &str) {
    match result {
        Ok(_) => panic!("expected a type error mentioning '{needle}', got Ok"),
        Err(e) => assert!(
            e.contains(needle),
            "error '{e}' does not mention '{needle}'"
        ),
    }
}

const POINT: &str = "class Point:
    var x: Int = 0;
    var y: Int = 0;
end";

mod class_declaration {
    use super::*;

    #[test]
    fn class_with_fields_is_accepted() {
        assert!(check(POINT).is_ok());
    }

    #[test]
    fn instantiation_has_class_type() {
        assert_eq!(type_of(POINT, "Point()"), Ok(Type::TClass("Point".into())));
    }

    #[test]
    fn variable_keeps_instance_type() {
        let decls = format!("{POINT}; val p = Point()");
        assert_eq!(type_of(&decls, "p"), Ok(Type::TClass("Point".into())));
    }

    #[test]
    fn instantiation_with_arguments_is_rejected() {
        assert_error_mentions(
            type_of(POINT, "Point(1)"),
            "class 'Point' has no constructor with arguments",
        );
    }

    #[test]
    fn unknown_class_instantiation_is_rejected() {
        assert_error_mentions(type_of(POINT, "Pont()"), "Pont");
    }

    #[test]
    fn duplicate_class_is_rejected() {
        assert_error_mentions(
            check("class A: end; class A: end"),
            "class 'A' is already declared",
        );
    }

    #[test]
    fn duplicate_field_is_rejected() {
        assert_error_mentions(
            check("class A: var x: Int = 0; var x: Int = 1; end"),
            "field 'x' is declared more than once",
        );
    }

    #[test]
    fn field_initializer_must_match_field_type() {
        assert_error_mentions(
            check("class A: var x: Int = True; end"),
            "field 'x' of class 'A' expects Int",
        );
    }

    #[test]
    fn field_with_unknown_type_is_rejected() {
        assert_error_mentions(check("class A: var n: Itn = 0; end"), "unknown class 'Itn'");
    }

    #[test]
    fn empty_list_and_nothing_fit_declared_field_types() {
        let src = "class Bag: var items: List[Int] = []; var best: Maybe[Int] = Nothing; end";
        assert!(check(src).is_ok());
    }

    #[test]
    fn field_may_hold_another_class() {
        let src = "class A: var v: Int = 1; end; class B: var a: A = A(); end";
        assert!(check(src).is_ok());
    }
}

mod methods {
    use super::*;

    #[test]
    fn method_with_self_typed_as_its_class_is_accepted() {
        let src = "class C: def get(self: C) -> Int: return 1; end; end";
        assert!(check(src).is_ok());
    }

    #[test]
    fn self_with_another_type_is_rejected() {
        let src = "class C: def get(self: Int) -> Int: return 1; end; end";
        assert_error_mentions(
            check(src),
            "first parameter of method 'get' must be 'self: C'",
        );
    }

    #[test]
    fn duplicate_method_is_rejected() {
        let src = "class C:
            def get(self: C) -> Int: return 1; end;
            def get(self: C) -> Int: return 2; end;
        end";
        assert_error_mentions(check(src), "method 'get' is declared more than once");
    }

    #[test]
    fn method_body_is_type_checked() {
        let src = "class C: def get(self: C) -> Int: return True; end; end";
        assert_error_mentions(check(src), "return type");
    }

    #[test]
    fn method_parameter_with_unknown_type_is_rejected() {
        let src = "class C: def f(self: C, o: Pont) -> Int: return 1; end; end";
        assert_error_mentions(check(src), "unknown class 'Pont'");
    }

    #[test]
    fn method_may_create_and_return_an_instance() {
        let src = "class C: def copy(self: C) -> C: return C(); end; end";
        assert!(check(src).is_ok());
    }

    #[test]
    fn methods_are_not_global_functions() {
        let src = "class C: def get(self: C) -> Int: return 1; end; end";
        assert!(type_of(src, "get(C())").is_err());
    }
}

mod class_types_outside_classes {
    use super::*;

    #[test]
    fn function_parameter_with_unknown_class_is_rejected() {
        assert_error_mentions(
            check("def f(p: Pont) -> Int: return 1; end"),
            "unknown class 'Pont'",
        );
    }

    #[test]
    fn function_return_with_unknown_class_is_rejected() {
        assert_error_mentions(
            check("def f() -> Pont: return 1; end"),
            "unknown class 'Pont'",
        );
    }

    #[test]
    fn function_body_can_instantiate_a_class() {
        let src = format!("{POINT}; def make() -> Point: return Point(); end");
        assert!(check(&src).is_ok());
    }
}

mod declaration_edge_cases {
    use super::*;

    #[test]
    fn class_name_cannot_collide_with_an_existing_function() {
        let src = "def Point() -> Int: return 1; end; class Point: end";
        assert_error_mentions(check(src), "class 'Point' conflicts with function 'Point'");
    }

    #[test]
    fn function_name_cannot_collide_with_an_existing_class() {
        let src = "class Point: end; def Point() -> Int: return 1; end";
        assert_error_mentions(check(src), "function 'Point' conflicts with class 'Point'");
    }

    #[test]
    fn class_name_cannot_collide_with_an_existing_variable() {
        let src = "val Point = 1; class Point: end";
        assert_error_mentions(check(src), "class 'Point' conflicts with variable 'Point'");
    }

    #[test]
    fn variable_name_cannot_shadow_an_existing_class() {
        let src = "class Point: end; val Point = 1";
        assert_error_mentions(check(src), "variable 'Point' conflicts with class 'Point'");
    }

    #[test]
    fn directly_recursive_default_is_rejected() {
        let src = "class Node: var next: Node = Node(); end";
        assert_error_mentions(check(src), "recursive field initializer");
    }

    #[test]
    fn mutually_recursive_defaults_are_rejected() {
        let src = "class A: var b: B = B(); end; class B: var a: A = A(); end";
        assert_error_mentions(check(src), "unknown class 'B'");
    }

    #[test]
    fn recursion_through_maybe_is_accepted() {
        let src = "class Node: var next: Maybe[Node] = Nothing; end";
        assert!(check(src).is_ok());
    }

    #[test]
    fn field_initializer_cannot_capture_a_global_variable() {
        let src = "val seed = 1; class Counter: var n: Int = seed; end";
        assert_error_mentions(
            check(src),
            "field initializer for 'Counter.n' must be closed",
        );
    }

    #[test]
    fn field_initializer_cannot_call_a_global_function() {
        let src = "def seed() -> Int: return 1; end; class Counter: var n: Int = seed(); end";
        assert_error_mentions(
            check(src),
            "field initializer for 'Counter.n' must be closed",
        );
    }

    #[test]
    fn field_initializer_may_use_constants_and_class_instantiation() {
        let src = "class A: var v: Int = 1 + 2; end; class B: var a: A = A(); end";
        assert!(check(src).is_ok());
    }

    #[test]
    fn nothing_may_be_returned_as_maybe() {
        let src = "def none() -> Maybe[Int]: return Nothing; end";
        assert!(check(src).is_ok());
    }

    #[test]
    fn empty_list_may_be_returned_as_typed_list() {
        let src = "def empty() -> List[Int]: return []; end";
        assert!(check(src).is_ok());
    }

    #[test]
    fn return_outside_function_is_a_type_error_not_a_panic() {
        assert_error_mentions(check("return 1"), "return outside function");
    }
}
const COUNTER: &str = "class Counter:
    var n: Int = 0;
    val step: Int = 1;
    def get(self: Counter) -> Int: return self.n; end;
    def add(self: Counter, k: Int) -> Unit: self.n = self.n + k; end;
    def twice(self: Counter) -> Int: return self.get() + self.get(); end;
end";

mod field_access {
    use super::*;

    #[test]
    fn field_access_has_the_field_type() {
        let decls = format!("{COUNTER}; val c = Counter()");
        assert_eq!(type_of(&decls, "c.n"), Ok(Type::TInteger));
    }

    #[test]
    fn field_access_on_an_instantiation() {
        assert_eq!(type_of(COUNTER, "Counter().step"), Ok(Type::TInteger));
    }

    #[test]
    fn field_access_composes_with_arithmetic() {
        let decls = format!("{COUNTER}; val c = Counter()");
        assert_eq!(type_of(&decls, "c.n + 1"), Ok(Type::TInteger));
    }

    #[test]
    fn missing_field_is_rejected() {
        let decls = format!("{COUNTER}; val c = Counter()");
        assert_error_mentions(type_of(&decls, "c.z"), "class 'Counter' has no field 'z'");
    }

    #[test]
    fn field_access_on_non_class_is_rejected() {
        assert_error_mentions(
            type_of("val i = 1", "i.n"),
            "only class instances have fields",
        );
    }

    #[test]
    fn chained_field_access_through_another_class() {
        let src = "class A: var v: Int = 1; end; class B: var a: A = A(); end; val b = B()";
        assert_eq!(type_of(src, "b.a.v"), Ok(Type::TInteger));
    }
}

mod method_call {
    use super::*;

    #[test]
    fn method_call_has_the_return_type() {
        let decls = format!("{COUNTER}; val c = Counter()");
        assert_eq!(type_of(&decls, "c.get()"), Ok(Type::TInteger));
    }

    #[test]
    fn method_call_does_not_pass_self_explicitly() {
        let decls = format!("{COUNTER}; val c = Counter()");
        assert_eq!(type_of(&decls, "c.add(2)"), Ok(Type::TVoid));
    }

    #[test]
    fn method_can_call_another_method_through_self() {
        let decls = format!("{COUNTER}; val c = Counter()");
        assert_eq!(type_of(&decls, "c.twice()"), Ok(Type::TInteger));
    }

    #[test]
    fn missing_method_is_rejected() {
        let decls = format!("{COUNTER}; val c = Counter()");
        assert_error_mentions(
            type_of(&decls, "c.reset()"),
            "class 'Counter' has no method 'reset'",
        );
    }

    #[test]
    fn wrong_argument_count_is_rejected() {
        let decls = format!("{COUNTER}; val c = Counter()");
        assert_error_mentions(
            type_of(&decls, "c.add()"),
            "method 'add' expects 1 argument(s), found 0",
        );
    }

    #[test]
    fn wrong_argument_type_is_rejected() {
        let decls = format!("{COUNTER}; val c = Counter()");
        assert_error_mentions(
            type_of(&decls, "c.add(True)"),
            "argument 1 of method 'add' expects Int, found Boolean",
        );
    }

    #[test]
    fn method_call_on_non_class_is_rejected() {
        assert_error_mentions(
            type_of("val i = 1", "i.get()"),
            "only class instances have methods",
        );
    }

    #[test]
    fn method_body_using_missing_field_is_rejected() {
        let src = "class C: def get(self: C) -> Int: return self.z; end; end";
        assert_error_mentions(check(src), "class 'C' has no field 'z'");
    }
}

mod field_assignment {
    use super::*;

    #[test]
    fn assigning_a_var_field_is_accepted() {
        let src = format!("{COUNTER}; val c = Counter(); c.n = 5");
        assert!(check(&src).is_ok());
    }

    #[test]
    fn assigning_a_val_field_is_rejected() {
        let src = format!("{COUNTER}; val c = Counter(); c.step = 5");
        assert_error_mentions(check(&src), "field 'step' of class 'Counter' is a val");
    }

    #[test]
    fn assigning_a_wrong_type_is_rejected() {
        let src = format!("{COUNTER}; val c = Counter(); c.n = True");
        assert_error_mentions(
            check(&src),
            "field 'n' of class 'Counter' expects Int, found Boolean",
        );
    }

    #[test]
    fn assigning_a_missing_field_is_rejected() {
        let src = format!("{COUNTER}; val c = Counter(); c.z = 1");
        assert_error_mentions(check(&src), "class 'Counter' has no field 'z'");
    }

    #[test]
    fn assigning_through_self_inside_a_method_is_accepted() {
        let src = "class C: var n: Int = 0; def set(self: C, v: Int) -> Unit: self.n = v; end; end";
        assert!(check(src).is_ok());
    }

    #[test]
    fn val_binding_does_not_freeze_var_fields() {
        let src = format!("{COUNTER}; val c = Counter(); c.n = 1; c.n = 2");
        assert!(check(&src).is_ok());
    }
}

#[test]
fn proposal_example_type_checks() {
    let src = std::fs::read_to_string("examples/classes/point_full.rpy").unwrap();
    assert!(check(&src).is_ok(), "{:?}", check(&src).err());
}

#[test]
fn declaration_example_type_checks() {
    let src = std::fs::read_to_string("examples/classes/point_declaration.rpy").unwrap();
    assert!(check(&src).is_ok(), "{:?}", check(&src).err());
}

/// Each program in examples/classes/type_errors must fail with its expected message.
#[test]
fn type_error_examples_fail_with_expected_messages() {
    let cases = [
        (
            "wrong_self_type.rpy",
            "first parameter of method 'area' must be 'self: Square'",
        ),
        ("unknown_type.rpy", "unknown class 'Pont'"),
        ("missing_field.rpy", "class 'Point' has no field 'z'"),
        (
            "val_field_assignment.rpy",
            "field 'id' of class 'Account' is a val",
        ),
        (
            "wrong_argument.rpy",
            "argument 1 of method 'add' expects Int, found String",
        ),
    ];
    for (file, needle) in cases {
        let src = std::fs::read_to_string(format!("examples/classes/type_errors/{file}"))
            .unwrap_or_else(|e| panic!("{file}: {e}"));
        assert_error_mentions(check(&src), needle);
    }
}
