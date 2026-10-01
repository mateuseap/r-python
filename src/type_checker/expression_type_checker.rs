use crate::environment::environment::{Environment, FuncOrVar};
use crate::ir::ast::{Expression, FuncSignature, Function, Name, Type};
type ErrorMessage = String;

pub fn check_expr(exp: Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    match exp {
        Expression::CTrue => Ok(Type::TBool),
        Expression::CFalse => Ok(Type::TBool),
        Expression::CVoid => Ok(Type::TVoid),
        Expression::CInt(_) => Ok(Type::TInteger),
        Expression::CReal(_) => Ok(Type::TReal),
        Expression::CString(_) => Ok(Type::TString),
        Expression::Add(l, r) => check_bin_arithmetic_expression(*l, *r, env),
        Expression::Sub(l, r) => check_bin_arithmetic_expression(*l, *r, env),
        Expression::Mul(l, r) => check_bin_arithmetic_expression(*l, *r, env),
        Expression::Div(l, r) => check_bin_arithmetic_expression(*l, *r, env),
        Expression::And(l, r) => check_bin_boolean_expression(*l, *r, env),
        Expression::Or(l, r) => check_bin_boolean_expression(*l, *r, env),
        Expression::Not(e) => check_not_expression(*e, env),
        Expression::EQ(l, r) => check_bin_relational_expression(*l, *r, env),
        Expression::NEQ(l, r) => check_bin_relational_expression(*l, *r, env),
        Expression::GT(l, r) => check_bin_relational_expression(*l, *r, env),
        Expression::LT(l, r) => check_bin_relational_expression(*l, *r, env),
        Expression::GTE(l, r) => check_bin_relational_expression(*l, *r, env),
        Expression::LTE(l, r) => check_bin_relational_expression(*l, *r, env),
        Expression::Var(name) => check_var_name(name, env),
        Expression::COk(e) => check_result_ok(*e, env),
        Expression::CErr(e) => check_result_err(*e, env),
        Expression::CJust(e) => check_maybe_just(*e, env),
        Expression::CNothing => Ok(Type::TMaybe(Box::new(Type::TAny))),
        Expression::IsError(e) => check_iserror_type(*e, env),
        Expression::IsNothing(e) => check_isnothing_type(*e, env),
        Expression::Unwrap(e) => check_unwrap_type(*e, env),
        Expression::Propagate(e) => check_propagate_type(*e, env),
        Expression::ListValue(elements) => check_list_value(&elements, env),
        Expression::Tuple(elements) => check_tuple_value(&elements, env),
        // Function call and lambda support from Sapienza branch
        Expression::FuncCall(func_name, exp_vec) => {
            check_func_call(func_name.clone(), exp_vec.clone(), env)
        }
        Expression::Lambda(func) => check_lambda(&func),
        Expression::Constructor(name, args) => check_adt_constructor(name, args, env),
        // Member access is parsed only; type checking it is future work.
        Expression::FieldAccess(_, name) | Expression::MethodCall(_, name, _) => Err(format!(
            "[Type Error] member access is not supported yet ('.{}')",
            name
        )),
    }
}

pub fn check_lambda(func: &Function) -> Result<Type, ErrorMessage> {
    Ok(func_to_type(func))
}

pub fn func_to_type(func: &Function) -> Type {
    let mut arg_types = Vec::new();
    for formal_arg in &func.params {
        arg_types.push(formal_arg.argument_type.clone());
    }
    Type::TFunction(Box::new(func.kind.clone()), arg_types)
}

pub fn check_func_call(
    func_name: Name,
    exp_vector: Vec<Expression>,
    env: &Environment<Type>,
) -> Result<Type, ErrorMessage> {
    let mut actual_arg_types = Vec::new();
    for arg in exp_vector.iter() {
        match arg {
            Expression::Var(name) => match env.lookup_var_or_func(name) {
                Some(FuncOrVar::Var((_, var_type))) => {
                    actual_arg_types.push(var_type);
                }
                Some(FuncOrVar::Func(func)) => {
                    actual_arg_types.push(func_to_type(&func));
                }
                None => {
                    return Err(format!("Identifier '{}' was never declared", name));
                }
            },
            _ => {
                let arg_type = check_expr(arg.clone(), env)?;
                actual_arg_types.push(arg_type);
            }
        }
    }
    let func_signature = FuncSignature {
        name: func_name.clone(),
        argument_types: actual_arg_types.clone(),
    };

    let func = env.lookup_function(&func_signature);
    if func.is_none() {
        if let Some(res) = check_builtin_signature(&func_name, &actual_arg_types) {
            return res;
        }

        return Err(format!(
            "Function {:?} was called but never declared",
            func_signature
        ));
    }
    let func = func.unwrap();

    let mut formal_arg_types = Vec::new();

    for param in func.params.iter() {
        formal_arg_types.push(param.argument_type.clone());
    }

    for (formal_type, actual_type) in formal_arg_types.iter().zip(actual_arg_types.iter()) {
        if formal_type != actual_type {
            return Err(format!(
                "Mismatched types in function {:?} call \n
            Expected:{:?}\n
            Received: {:?}",
                func_signature, formal_arg_types, actual_arg_types
            ));
        }
    }
    return Ok(func.kind.clone());
}

fn check_builtin_signature(
    func_name: &str,
    actual_arg_types: &[Type],
) -> Option<Result<Type, ErrorMessage>> {
    match func_name {
        "print" | "print_line" => {
            if actual_arg_types.len() != 1 {
                return Some(Err(format!(
                    "[Type Error] {} expects exactly 1 argument",
                    func_name
                )));
            }
            Some(Ok(Type::TVoid))
        }
        "input" => Some(Ok(Type::TString)),
        "input_int" => Some(Ok(Type::TInteger)),
        "input_real" => Some(Ok(Type::TReal)),
        "to_string" => {
            if actual_arg_types.len() != 1 {
                return Some(Err(
                    "[Type Error] to_string expects exactly 1 argument".to_string()
                ));
            }
            Some(Ok(Type::TString))
        }
        "to_string_fixed" => {
            if actual_arg_types.len() != 2 {
                return Some(Err(
                    "[Type Error] to_string_fixed expects exactly 2 arguments".to_string(),
                ));
            }
            Some(Ok(Type::TString))
        }
        "str_concat" => {
            if actual_arg_types.len() != 2 {
                return Some(Err(
                    "[Type Error] str_concat expects exactly 2 arguments".to_string()
                ));
            }
            Some(Ok(Type::TString))
        }
        "open" => {
            if actual_arg_types.is_empty() || actual_arg_types.len() > 3 {
                return Some(Err("[Type Error] open expects 1 to 3 arguments".to_string()));
            }
            Some(Ok(Type::TAny))
        }
        "len" => {
            if actual_arg_types.len() != 1 {
                return Some(Err(
                    "[Type Error] len expects exactly 1 argument".to_string()
                ));
            }
            match &actual_arg_types[0] {
                Type::TString | Type::TList(_) | Type::TTuple(_) | Type::TAny => {
                    Some(Ok(Type::TInteger))
                }
                other => Some(Err(format!(
                    "[Type Error] len expected string or list, found {:?}",
                    other
                ))),
            }
        }
        "join" => {
            if actual_arg_types.len() != 2 {
                return Some(Err(
                    "[Type Error] join expects exactly 2 arguments".to_string()
                ));
            }
            let coll_type = &actual_arg_types[0];
            let sep_type = &actual_arg_types[1];
            let sep_ok = matches!(sep_type, Type::TString | Type::TAny);
            let coll_ok = match coll_type {
                Type::TList(inner) => **inner == Type::TString || **inner == Type::TAny,
                Type::TAny => true,
                _ => false,
            };

            if !coll_ok {
                return Some(Err(format!(
                    "[Type Error] join expects a list of strings as first argument, found {:?}",
                    coll_type
                )));
            }
            if !sep_ok {
                return Some(Err(format!(
                    "[Type Error] join expects a string separator, found {:?}",
                    sep_type
                )));
            }
            Some(Ok(Type::TString))
        }
        "to_int" => {
            if actual_arg_types.len() != 1 {
                return Some(Err(
                    "[Type Error] to_int expects exactly 1 argument".to_string()
                ));
            }
            Some(Ok(Type::TInteger))
        }
        "to_real" => {
            if actual_arg_types.len() != 1 {
                return Some(Err(
                    "[Type Error] to_real expects exactly 1 argument".to_string()
                ));
            }
            Some(Ok(Type::TReal))
        }
        _ => None,
    }
}

fn check_var_name(name: Name, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    match env.lookup(&name) {
        Some((_, t)) => Ok(t.clone()),
        None => Err(format!("[Name Error] '{}' is not defined.", name)),
    }
}

fn check_bin_arithmetic_expression(
    left: Expression,
    right: Expression,
    env: &Environment<Type>,
) -> Result<Type, ErrorMessage> {
    let left_type = check_expr(left, env)?;
    let right_type = check_expr(right, env)?;

    match (left_type, right_type) {
        (Type::TInteger, Type::TInteger) => Ok(Type::TInteger),
        (Type::TInteger, Type::TReal) => Ok(Type::TReal),
        (Type::TReal, Type::TInteger) => Ok(Type::TReal),
        (Type::TReal, Type::TReal) => Ok(Type::TReal),
        _ => Err(String::from("[Type Error] expecting numeric type values.")),
    }
}

fn check_bin_boolean_expression(
    left: Expression,
    right: Expression,
    env: &Environment<Type>,
) -> Result<Type, ErrorMessage> {
    let left_type = check_expr(left, env)?;
    let right_type = check_expr(right, env)?;
    match (left_type, right_type) {
        (Type::TBool, Type::TBool) => Ok(Type::TBool),
        _ => Err(String::from("[Type Error] expecting boolean type values.")),
    }
}

fn check_not_expression(exp: Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    let exp_type = check_expr(exp, env)?;

    match exp_type {
        Type::TBool => Ok(Type::TBool),
        _ => Err(String::from("[Type Error] expecting a boolean type value.")),
    }
}

fn check_bin_relational_expression(
    left: Expression,
    right: Expression,
    env: &Environment<Type>,
) -> Result<Type, ErrorMessage> {
    let left_type = check_expr(left, env)?;
    let right_type = check_expr(right, env)?;

    match (left_type, right_type) {
        (Type::TInteger, Type::TInteger) => Ok(Type::TBool),
        (Type::TInteger, Type::TReal) => Ok(Type::TBool),
        (Type::TReal, Type::TInteger) => Ok(Type::TBool),
        (Type::TReal, Type::TReal) => Ok(Type::TBool),
        _ => Err(String::from("[Type Error] expecting numeric type values.")),
    }
}

fn check_result_ok(exp: Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    let exp_type = check_expr(exp, env)?;
    return Ok(Type::TResult(Box::new(exp_type), Box::new(Type::TAny)));
}

fn check_result_err(exp: Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    let exp_type = check_expr(exp, env)?;
    return Ok(Type::TResult(Box::new(Type::TAny), Box::new(exp_type)));
}

fn check_unwrap_type(exp: Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    let exp_type = check_expr(exp, env)?;

    match exp_type {
        Type::TMaybe(t) => Ok(*t),
        Type::TResult(tl, _) => Ok(*tl),
        _ => Err(String::from(
            "[Type Error] expecting a maybe or result type value.",
        )),
    }
}

fn check_propagate_type(exp: Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    let exp_type = check_expr(exp, env)?;

    match exp_type {
        Type::TMaybe(t) => Ok(*t),
        Type::TResult(tl, _) => Ok(*tl),
        _ => Err(String::from(
            "[Type Error] expecting a maybe or result type value.",
        )),
    }
}

fn check_maybe_just(exp: Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    let exp_type = check_expr(exp, env)?;
    Ok(Type::TMaybe(Box::new(exp_type)))
}

fn check_iserror_type(exp: Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    let v = check_expr(exp, env)?;

    match v {
        Type::TResult(_, _) => Ok(Type::TBool),
        _ => Err(String::from("[Type Error] expecting a result type value.")),
    }
}

fn check_isnothing_type(exp: Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    let exp_type = check_expr(exp, env)?;

    match exp_type {
        Type::TMaybe(_) => Ok(Type::TBool),
        _ => Err(String::from("[Type Error] expecting a maybe type value.")),
    }
}

fn check_list_value(
    elements: &[Expression],
    env: &Environment<Type>,
) -> Result<Type, ErrorMessage> {
    if elements.is_empty() {
        return Ok(Type::TList(Box::new(Type::TAny)));
    }

    // Check the type of the first element
    let first_type = check_expr(elements[0].clone(), env)?;

    // Check that all other elements have the same type
    for element in elements.iter().skip(1) {
        let element_type = check_expr(element.clone(), env)?;
        if element_type != first_type {
            return Err(format!(
                "[Type Error] List elements must have the same type. Expected '{:?}', found '{:?}'.",
                first_type, element_type
            ));
        }
    }

    Ok(Type::TList(Box::new(first_type)))
}

fn check_tuple_value(
    elements: &[Expression],
    env: &Environment<Type>,
) -> Result<Type, ErrorMessage> {
    let mut types = Vec::with_capacity(elements.len());
    for element in elements {
        let element_type = check_expr(element.clone(), env)?;
        types.push(element_type);
    }
    Ok(Type::TTuple(types))
}

fn check_adt_constructor(
    name: Name,
    args: Vec<Box<Expression>>,
    env: &Environment<Type>,
) -> Result<Type, ErrorMessage> {
    // Gather all ADTs from all scopes (stack and globals)
    let mut found = None;
    // Search stack scopes first (innermost to outermost)
    for scope in env.stack.iter() {
        for (adt_name, constructors) in scope.adts.iter() {
            if let Some(constructor) = constructors.iter().find(|c| c.name == name) {
                found = Some((adt_name.clone(), constructor.clone(), constructors.clone()));
                break;
            }
        }
        if found.is_some() {
            break;
        }
    }
    // If not found in stack, search globals
    if found.is_none() {
        for (adt_name, constructors) in env.globals.adts.iter() {
            if let Some(constructor) = constructors.iter().find(|c| c.name == name) {
                found = Some((adt_name.clone(), constructor.clone(), constructors.clone()));
                break;
            }
        }
    }
    match found {
        Some((adt_type_name, constructor, constructors)) => {
            // Check that we have the right number of arguments
            if args.len() != constructor.types.len() {
                return Err(format!(
                    "[Type Error] Constructor '{}' expects {} arguments, but got {}.",
                    name,
                    constructor.types.len(),
                    args.len()
                ));
            }
            // Check each argument's type
            for (arg, expected_type) in args.iter().zip(constructor.types.iter()) {
                let arg_type = check_expr(*arg.clone(), env)?;
                if arg_type != *expected_type {
                    return Err(format!(
                        "[Type Error] Argument type mismatch in constructor '{}'. Expected '{:?}', found '{:?}'.",
                        name, expected_type, arg_type
                    ));
                }
            }
            // Return the algebraic type
            Ok(Type::TAlgebraicData(adt_type_name, constructors))
        }
        None => Err(format!(
            "[Type Error] Constructor '{}' is not defined in any ADT.",
            name
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::environment::Environment;
    use crate::ir::ast::Expression::*;
    use crate::ir::ast::Type::*;
    use crate::ir::ast::{Type, ValueConstructor};

    #[test]
    fn check_constant() {
        let env = Environment::new();
        let c10 = CInt(10);

        assert_eq!(check_expr(c10, &env), Ok(TInteger));
    }

    #[test]
    fn check_add_integers() {
        let env = Environment::new();

        let c10 = CInt(10);
        let c20 = CInt(20);
        let add = Add(Box::new(c10), Box::new(c20));

        assert_eq!(check_expr(add, &env), Ok(TInteger));
    }

    #[test]
    fn check_add_reals() {
        let env = Environment::new();

        let c10 = CReal(10.5);
        let c20 = CReal(20.3);
        let add = Add(Box::new(c10), Box::new(c20));

        assert_eq!(check_expr(add, &env), Ok(TReal));
    }

    #[test]
    fn check_add_real_and_integer() {
        let env = Environment::new();

        let c10 = CInt(10);
        let c20 = CReal(20.3);
        let add = Add(Box::new(c10), Box::new(c20));

        assert_eq!(check_expr(add, &env), Ok(TReal));
    }

    #[test]
    fn check_add_integer_and_real() {
        let env = Environment::new();

        let c10 = CReal(10.5);
        let c20 = CInt(20);
        let add = Add(Box::new(c10), Box::new(c20));

        assert_eq!(check_expr(add, &env), Ok(TReal));
    }

    #[test]
    fn check_type_error_arithmetic_expression() {
        let env = Environment::new();

        let e1 = CInt(10);
        let e2 = CFalse;
        let e3 = Add(Box::new(e1), Box::new(e2));

        assert!(
            matches!(check_expr(e3, &env), Err(_)),
            "Expecting a type error."
        );
    }

    #[test]
    fn check_type_error_not_expression() {
        let env = Environment::new();

        let e1 = CInt(10);
        let e2 = Not(Box::new(e1));

        assert!(
            matches!(check_expr(e2, &env), Err(_)),
            "Expecting a type error."
        );
    }

    #[test]
    fn check_type_error_and_expression() {
        let env = Environment::new();

        let e1 = CInt(10);
        let e2 = CTrue;
        let e3 = And(Box::new(e1), Box::new(e2));

        assert!(
            matches!(check_expr(e3, &env), Err(_)),
            "Expecting a type error."
        );
    }

    #[test]
    fn check_type_error_or_expression() {
        let env = Environment::new();

        let e1 = CInt(10);
        let e2 = CTrue;
        let e3 = Or(Box::new(e1), Box::new(e2));

        assert!(
            matches!(check_expr(e3, &env), Err(_)),
            "Expecting a type error."
        );
    }

    #[test]
    fn check_ok_result() {
        let env = Environment::new();
        let e1 = CReal(10.0);
        let e2 = COk(Box::new(e1));

        assert_eq!(
            check_expr(e2, &env),
            Ok(TResult(Box::new(TReal), Box::new(TAny)))
        );
    }

    #[test]
    fn check_err_result() {
        let env = Environment::new();
        let e1 = CInt(1);
        let e2 = CErr(Box::new(e1));

        assert_eq!(
            check_expr(e2, &env),
            Ok(TResult(Box::new(TAny), Box::new(TInteger)))
        );
    }

    #[test]
    fn check_just_integer() {
        let env = Environment::new();
        let e1 = CInt(5);
        let e2 = CJust(Box::new(e1));

        assert_eq!(check_expr(e2, &env), Ok(TMaybe(Box::new(TInteger))))
    }

    #[test]
    fn check_is_error_result_positive() {
        let env = Environment::new();
        let e1 = CTrue;
        let e2 = COk(Box::new(e1));
        let e3 = IsError(Box::new(e2));

        assert_eq!(check_expr(e3, &env), Ok(TBool));
    }

    #[test]
    fn check_is_error_result_error() {
        let env = Environment::new();
        let e1 = CTrue;
        let e2 = IsError(Box::new(e1));

        assert!(
            matches!(check_expr(e2, &env), Err(_)),
            "Expecting a result type value."
        );
    }

    #[test]
    fn check_nothing() {
        let env = Environment::new();

        assert_eq!(check_expr(CNothing, &env), Ok(TMaybe(Box::new(TAny))));
    }

    #[test]
    fn check_is_nothing_on_maybe() {
        let env = Environment::new();
        let e1 = CInt(5);
        let e2 = CJust(Box::new(e1));
        let e3 = IsNothing(Box::new(e2));

        assert_eq!(check_expr(e3, &env), Ok(TBool));
    }

    #[test]
    fn check_is_nothing_type_error() {
        let env = Environment::new();
        let e1 = CInt(5);
        let e2 = IsNothing(Box::new(e1));

        assert!(
            matches!(check_expr(e2, &env), Err(_)),
            "expecting a maybe type value."
        );
    }

    #[test]
    fn check_unwrap_maybe() {
        let env = Environment::new();
        let e1 = CInt(5);
        let e2 = CJust(Box::new(e1));
        let e3 = Unwrap(Box::new(e2));

        assert_eq!(check_expr(e3, &env), Ok(TInteger));
    }

    #[test]
    fn check_unwrap_maybe_type_error() {
        let env = Environment::new();
        let e1 = CInt(5);
        let e2 = Unwrap(Box::new(e1));

        assert!(
            matches!(check_expr(e2, &env), Err(_)),
            "expecting a maybe or result type value."
        );
    }

    #[test]
    fn check_unwrap_result() {
        let env = Environment::new();
        let e1 = CTrue;
        let e2 = COk(Box::new(e1));
        let e3 = Unwrap(Box::new(e2));

        assert_eq!(check_expr(e3, &env), Ok(TBool));
    }

    #[test]
    fn check_propagate_maybe() {
        let env = Environment::new();
        let e1 = CInt(5);
        let e2 = CJust(Box::new(e1));
        let e3 = Propagate(Box::new(e2));

        assert_eq!(check_expr(e3, &env), Ok(TInteger));
    }

    #[test]
    fn check_propagate_maybe_type_error() {
        let env = Environment::new();
        let e1 = CInt(5);
        let e2 = Propagate(Box::new(e1));

        assert!(
            matches!(check_expr(e2, &env), Err(_)),
            "expecting a maybe or result type value."
        );
    }

    #[test]
    fn check_propagate_result() {
        let env = Environment::new();
        let e1 = CTrue;
        let e2 = COk(Box::new(e1));
        let e3 = Propagate(Box::new(e2));

        assert_eq!(check_expr(e3, &env), Ok(TBool));
    }

    #[test]
    fn test_undefined_variable() {
        let env = Environment::new();
        let exp = Expression::Var("x".to_string());

        // Should fail - x is not defined
        assert!(check_expr(exp, &env).is_err());
    }

    #[test]
    fn test_defined_variable() {
        let mut env = Environment::new();
        env.map_variable("x".to_string(), true, Type::TInteger);
        let exp = Expression::Var("x".to_string());

        // Should succeed and return integer type
        assert_eq!(check_expr(exp, &env), Ok(Type::TInteger));
    }

    #[test]
    fn test_adt_constructor_valid() {
        let mut env = Environment::new();
        let figure_type = vec![
            ValueConstructor::new("Circle".to_string(), vec![Type::TInteger]),
            ValueConstructor::new(
                "Rectangle".to_string(),
                vec![Type::TInteger, Type::TInteger],
            ),
        ];
        env.map_adt("Figure".to_string(), figure_type);

        let circle = Constructor("Circle".to_string(), vec![Box::new(CInt(5))]);
        let result = check_expr(circle, &env);
        assert!(result.is_ok());
    }

    #[test]
    fn test_adt_constructor_wrong_args() {
        let mut env = Environment::new();
        let figure_type = vec![
            ValueConstructor::new("Circle".to_string(), vec![Type::TInteger]),
            ValueConstructor::new(
                "Rectangle".to_string(),
                vec![Type::TInteger, Type::TInteger],
            ),
        ];
        env.map_adt("Figure".to_string(), figure_type);

        let circle = Constructor(
            "Circle".to_string(),
            vec![Box::new(CString("invalid".to_string()))],
        );
        let result = check_expr(circle, &env);
        assert!(result.is_err());
    }

    #[test]
    fn test_adt_constructor_wrong_count() {
        let mut env = Environment::new();
        let figure_type = vec![
            ValueConstructor::new("Circle".to_string(), vec![Type::TInteger]),
            ValueConstructor::new(
                "Rectangle".to_string(),
                vec![Type::TInteger, Type::TInteger],
            ),
        ];
        env.map_adt("Figure".to_string(), figure_type);

        let rectangle = Constructor("Rectangle".to_string(), vec![Box::new(CInt(5))]); // Missing second argument
        let result = check_expr(rectangle, &env);
        assert!(result.is_err());
    }

    #[test]
    fn test_adt_constructor_undefined() {
        let env = Environment::new();
        let circle = Constructor("Circle".to_string(), vec![Box::new(CInt(5))]);
        let result = check_expr(circle, &env);
        assert!(result.is_err());
    }

    #[test]
    fn test_adt_constructor_with_mutable_vars() {
        let mut env = Environment::new();
        let figure_type = vec![
            ValueConstructor::new("Circle".to_string(), vec![Type::TInteger]),
            ValueConstructor::new(
                "Rectangle".to_string(),
                vec![Type::TInteger, Type::TInteger],
            ),
        ];
        env.map_adt("Figure".to_string(), figure_type);

        // Create a mutable variable to use in constructor
        env.map_variable("radius".to_string(), true, Type::TInteger);

        let circle = Constructor(
            "Circle".to_string(),
            vec![Box::new(Var("radius".to_string()))],
        );
        let result = check_expr(circle, &env);
        assert!(result.is_ok());
    }
}
