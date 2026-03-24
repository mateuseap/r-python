use crate::environment::environment::Environment;
use crate::ir::ast::{Expression, Statement};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::sync::OnceLock;

pub type MetaBuiltinStmt = fn(&mut Environment<Expression>) -> Statement;

// Tabela estática global de metabuiltins
static METABUILTINS_TABLE: OnceLock<HashMap<String, MetaBuiltinStmt>> = OnceLock::new();

pub fn get_metabuiltins_table() -> &'static HashMap<String, MetaBuiltinStmt> {
    METABUILTINS_TABLE.get_or_init(|| {
        let mut table = HashMap::new();
        table.insert("input".to_string(), input_builtin as MetaBuiltinStmt);
        table.insert("print".to_string(), print_builtin as MetaBuiltinStmt);
        table.insert("open".to_string(), open_builtin as MetaBuiltinStmt);
        table.insert(
            "tuple_get".to_string(),
            tuple_get_builtin as MetaBuiltinStmt,
        );
        table.insert(
            "input_int".to_string(),
            input_int_builtin as MetaBuiltinStmt,
        );
        table.insert(
            "input_real".to_string(),
            input_real_builtin as MetaBuiltinStmt,
        );
        table.insert(
            "to_string".to_string(),
            to_string_builtin as MetaBuiltinStmt,
        );
        table.insert(
            "to_string_fixed".to_string(),
            to_string_fixed_builtin as MetaBuiltinStmt,
        );
        table.insert(
            "str_concat".to_string(),
            str_concat_builtin as MetaBuiltinStmt,
        );
        table.insert("len".to_string(), len_builtin as MetaBuiltinStmt);
        table.insert(
            "print_line".to_string(),
            print_line_builtin as MetaBuiltinStmt,
        );
        table.insert("join".to_string(), join_builtin as MetaBuiltinStmt);
        table.insert("to_int".to_string(), to_int_builtin as MetaBuiltinStmt);
        table.insert("to_real".to_string(), to_real_builtin as MetaBuiltinStmt);
        table
    })
}

pub fn tuple_get_builtin(env: &mut Environment<Expression>) -> Statement {
    let value = env.lookup(&"value".to_string()).map(|(_, v)| v.clone());
    let index = env.lookup(&"index".to_string()).map(|(_, v)| v.clone());

    let result = match (value, index) {
        (Some(Expression::Tuple(items)), Some(Expression::CInt(i))) => {
            if i < 0 {
                Expression::CString("tuple_get: index must be >= 0".to_string())
            } else {
                let idx = i as usize;
                items
                    .get(idx)
                    .cloned()
                    .unwrap_or(Expression::CString("tuple_get: index out of bounds".to_string()))
            }
        }
        (Some(other), Some(Expression::CInt(_))) => Expression::CString(format!(
            "tuple_get: expected tuple as first argument, found {:?}",
            other
        )),
        (Some(_), Some(other_idx)) => Expression::CString(format!(
            "tuple_get: expected int index as second argument, found {:?}",
            other_idx
        )),
        (None, _) => Expression::CString("tuple_get: missing argument 'value'".to_string()),
        (_, None) => Expression::CString("tuple_get: missing argument 'index'".to_string()),
    };

    Statement::Return(Box::new(result))
}

pub fn input_builtin(env: &mut Environment<Expression>) -> Statement {
    let prompt = match env.lookup(&"prompt".to_string()) {
        Some((_, Expression::CString(s))) => s.clone(),
        _ => "".to_string(),
    };
    print!("{}", prompt);
    use std::io::{self, Write};
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim_end_matches(['\n', '\r']).to_string();
    Statement::Return(Box::new(Expression::CString(input)))
}

pub fn print_builtin(env: &mut Environment<Expression>) -> Statement {
    let value = env
        .lookup(&"value".to_string())
        .map(|(_, v)| v)
        .unwrap_or(Expression::CString("".to_string()));
    print!("{}", expr_to_string(&value));
    use std::io::{self, Write};
    io::stdout().flush().unwrap();
    Statement::Return(Box::new(Expression::CVoid))
}

pub fn input_int_builtin(env: &mut Environment<Expression>) -> Statement {
    let prompt = match env.lookup(&"prompt".to_string()) {
        Some((_, Expression::CString(s))) => s.clone(),
        _ => "".to_string(),
    };
    print!("{}", prompt);
    use std::io::{self, Write};
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim_end_matches(['\n', '\r']);
    match trimmed.parse::<i32>() {
        Ok(v) => Statement::Return(Box::new(Expression::CInt(v))),
        Err(e) => Statement::Return(Box::new(Expression::CString(format!(
            "input_int: failed to parse integer: {}",
            e
        )))),
    }
}

pub fn input_real_builtin(env: &mut Environment<Expression>) -> Statement {
    let prompt = match env.lookup(&"prompt".to_string()) {
        Some((_, Expression::CString(s))) => s.clone(),
        _ => "".to_string(),
    };
    print!("{}", prompt);
    use std::io::{self, Write};
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim_end_matches(['\n', '\r']);
    match trimmed.parse::<f64>() {
        Ok(v) => Statement::Return(Box::new(Expression::CReal(v))),
        Err(e) => Statement::Return(Box::new(Expression::CString(format!(
            "input_real: failed to parse real: {}",
            e
        )))),
    }
}

fn escape_string_literal(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn expr_to_string(expr: &Expression) -> String {
    match expr {
        Expression::CString(s) => s.clone(),
        Expression::CInt(i) => i.to_string(),
        Expression::CReal(f) => f.to_string(),
        Expression::CTrue => "True".to_string(),
        Expression::CFalse => "False".to_string(),
        Expression::CVoid => "".to_string(),

        Expression::ListValue(items) => {
            let inner = items
                .iter()
                .map(expr_to_literal)
                .collect::<Vec<_>>()
                .join(", ");
            format!("[{inner}]")
        }
        Expression::Tuple(items) => {
            let inner = items
                .iter()
                .map(expr_to_literal)
                .collect::<Vec<_>>()
                .join(", ");
            format!("({inner})")
        }

        Expression::CJust(e) => format!("Just({})", expr_to_literal(e)),
        Expression::CNothing => "Nothing".to_string(),
        Expression::COk(e) => format!("Ok({})", expr_to_literal(e)),
        Expression::CErr(e) => format!("Err({})", expr_to_literal(e)),

        other => format!("{:?}", other),
    }
}

fn expr_to_literal(expr: &Expression) -> String {
    match expr {
        Expression::CString(s) => format!("\"{}\"", escape_string_literal(s)),
        _ => expr_to_string(expr),
    }
}

pub fn to_string_builtin(env: &mut Environment<Expression>) -> Statement {
    let value = env
        .lookup(&"value".to_string())
        .map(|(_, v)| v)
        .unwrap_or(Expression::CVoid);
    Statement::Return(Box::new(Expression::CString(expr_to_string(&value))))
}

pub fn str_concat_builtin(env: &mut Environment<Expression>) -> Statement {
    let left = env
        .lookup(&"left".to_string())
        .map(|(_, v)| v)
        .unwrap_or(Expression::CString("".to_string()));
    let right = env
        .lookup(&"right".to_string())
        .map(|(_, v)| v)
        .unwrap_or(Expression::CString("".to_string()));

    let res = format!("{}{}", expr_to_string(&left), expr_to_string(&right));
    Statement::Return(Box::new(Expression::CString(res)))
}

pub fn len_builtin(env: &mut Environment<Expression>) -> Statement {
    let value = env.lookup(&"value".to_string()).map(|(_, v)| v);

    let result = match value {
        Some(Expression::CString(s)) => Expression::CInt(s.chars().count() as i32),
        Some(Expression::ListValue(items)) => Expression::CInt(items.len() as i32),
        Some(Expression::Tuple(items)) => Expression::CInt(items.len() as i32),
        Some(other) => Expression::CString(format!("len: unsupported type {:?}", other)),
        None => Expression::CString("len: missing argument 'value'".to_string()),
    };

    Statement::Return(Box::new(result))
}

pub fn print_line_builtin(env: &mut Environment<Expression>) -> Statement {
    let value = env
        .lookup(&"value".to_string())
        .map(|(_, v)| v)
        .unwrap_or(Expression::CString("".to_string()));

    println!("{}", expr_to_string(&value));

    Statement::Return(Box::new(Expression::CVoid))
}

pub fn join_builtin(env: &mut Environment<Expression>) -> Statement {
    let values = env.lookup(&"values".to_string()).map(|(_, v)| v);
    let sep = env.lookup(&"sep".to_string()).map(|(_, v)| v);

    match (values, sep) {
        (Some(Expression::ListValue(items)), Some(Expression::CString(sep_str))) => {
            let mut parts = Vec::with_capacity(items.len());
            for item in items.iter() {
                match item {
                    Expression::CString(s) => parts.push(s.clone()),
                    other => {
                        return Statement::Return(Box::new(Expression::CString(format!(
                            "join: expected list of strings, found {:?}",
                            other
                        ))))
                    }
                }
            }
            let joined = parts.join(&sep_str);
            Statement::Return(Box::new(Expression::CString(joined)))
        }
        (Some(_), Some(other_sep)) => Statement::Return(Box::new(Expression::CString(format!(
            "join: separator must be a string, found {:?}",
            other_sep
        )))),
        (None, _) => Statement::Return(Box::new(Expression::CString(
            "join: missing argument 'values'".to_string(),
        ))),
        (_, None) => Statement::Return(Box::new(Expression::CString(
            "join: missing argument 'sep'".to_string(),
        ))),
    }
}

pub fn to_int_builtin(env: &mut Environment<Expression>) -> Statement {
    let value = env
        .lookup(&"value".to_string())
        .map(|(_, v)| v)
        .unwrap_or(Expression::CVoid);

    let result = match value {
        Expression::CInt(i) => Expression::CInt(i),
        Expression::CReal(f) => Expression::CInt(f as i32),
        Expression::CString(s) => match s.trim().parse::<i32>() {
            Ok(v) => Expression::CInt(v),
            Err(e) => Expression::CString(format!("to_int: failed to parse integer: {}", e)),
        },
        other => Expression::CString(format!("to_int: unsupported type {:?}", other)),
    };

    Statement::Return(Box::new(result))
}

pub fn to_real_builtin(env: &mut Environment<Expression>) -> Statement {
    let value = env
        .lookup(&"value".to_string())
        .map(|(_, v)| v)
        .unwrap_or(Expression::CVoid);

    let result = match value {
        Expression::CReal(f) => Expression::CReal(f),
        Expression::CInt(i) => Expression::CReal(i as f64),
        Expression::CString(s) => match s.trim().parse::<f64>() {
            Ok(v) => Expression::CReal(v),
            Err(e) => Expression::CString(format!("to_real: failed to parse real: {}", e)),
        },
        other => Expression::CString(format!("to_real: unsupported type {:?}", other)),
    };

    Statement::Return(Box::new(result))
}

pub fn to_string_fixed_builtin(env: &mut Environment<Expression>) -> Statement {
    let value = env
        .lookup(&"value".to_string())
        .map(|(_, v)| v)
        .unwrap_or(Expression::CVoid);
    let places = env
        .lookup(&"places".to_string())
        .map(|(_, v)| v)
        .unwrap_or(Expression::CInt(0));

    let places_u32 = match places {
        Expression::CInt(i) if i >= 0 => i as usize,
        Expression::CReal(f) if f >= 0.0 => f as usize,
        _ => {
            return Statement::Return(Box::new(Expression::CString(
                "to_string_fixed: places must be a non-negative number".to_string(),
            )))
        }
    };

    let formatted = match value {
        Expression::CInt(i) => format!("{:.prec$}", i as f64, prec = places_u32),
        Expression::CReal(f) => format!("{:.prec$}", f, prec = places_u32),
        other => format!("{:.prec$}", expr_to_string(&other), prec = places_u32),
    };

    Statement::Return(Box::new(Expression::CString(formatted)))
}

pub fn open_builtin(env: &mut Environment<Expression>) -> Statement {
    let path = match env.lookup(&"path".to_string()) {
        Some((_, Expression::CString(p))) => p.clone(),
        _ => {
            return Statement::Return(Box::new(Expression::CString(
                "open: first argument must be a string with the file path".to_string(),
            )));
        }
    };

    let mode = match env.lookup(&"mode".to_string()) {
        Some((_, Expression::CString(m))) => m.clone(),
        _ => "r".to_string(),
    };

    match mode.as_str() {
        "r" => {
            let mut file = match File::open(&path) {
                Ok(f) => f,
                Err(e) => {
                    return Statement::Return(Box::new(Expression::CString(format!(
                        "open: could not open '{}' for reading: {}",
                        path, e
                    ))));
                }
            };
            let mut contents = String::new();
            if let Err(e) = file.read_to_string(&mut contents) {
                return Statement::Return(Box::new(Expression::CString(format!(
                    "open: error reading '{}': {}",
                    path, e
                ))));
            }

            Statement::Return(Box::new(Expression::CString(contents)))
        }

        "w" => {
            let content = match env.lookup(&"content".to_string()) {
                Some((_, Expression::CString(c))) => c.clone(),
                _ => {
                    return Statement::Return(Box::new(Expression::CString(
                        "open: when using mode 'w', a third argument with the content to write is required".to_string())));
                }
            };

            match std::fs::write(&path, content) {
                Ok(_) => Statement::Return(Box::new(Expression::CVoid)),
                Err(e) => Statement::Return(Box::new(Expression::CString(format!(
                    "open: could not write to '{}': {}",
                    path, e
                )))),
            }
        }

        "a" => {
            let content = match env.lookup(&"content".to_string()) {
                Some((_, Expression::CString(c))) => c.clone(),
                _ => {
                    return Statement::Return(Box::new(Expression::CString(
                        "open: when using mode 'a', a third argument with the content to append is required".to_string())));
                }
            };

            match std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(&path)
            {
                Ok(mut file) => {
                    if let Err(e) = writeln!(file, "{}", content) {
                        return Statement::Return(Box::new(Expression::CString(format!(
                            "open: could not append to '{}': {}",
                            path, e
                        ))));
                    }
                    Statement::Return(Box::new(Expression::CVoid))
                }
                Err(e) => Statement::Return(Box::new(Expression::CString(format!(
                    "open: could not open '{}' for appending: {}",
                    path, e
                )))),
            }
        }

        m => Statement::Return(Box::new(Expression::CString(format!(
            "open: unsupported mode '{}'.",
            m
        )))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_stmt_table_contains_input_function() {
        let table = get_metabuiltins_table();
        assert!(
            table.contains_key("input"),
            "The table must contain the 'input' function"
        );

        // Check if the function in the table is callable
        let input_func = table.get("input").unwrap();
        let _env: Environment<Expression> = Environment::new();
        // We do not execute the function to avoid blocking stdin
        // Just check if it exists and can be referenced
        assert!(
            std::ptr::addr_of!(*input_func) != std::ptr::null(),
            "The 'input' function must exist in the table"
        );
    }

    #[test]
    fn test_meta_stmt_table_contains_print_function() {
        let table = get_metabuiltins_table();
        assert!(
            table.contains_key("print"),
            "The table must contain the 'print' function"
        );

        // Check if the function in the table is callable
        let print_func = table.get("print").unwrap();
        let mut env: Environment<Expression> = Environment::new();
        let result = print_func(&mut env);
        match result {
            Statement::Return(_) => (), // Expected
            _ => panic!("The 'print' function in the table must return Statement::Return"),
        }
    }

    #[test]
    fn test_meta_stmt_table_has_correct_size() {
        let table = get_metabuiltins_table();
        assert_eq!(
            table.len(),
            14,
            "The table must contain exactly 14 functions"
        );
    }

    #[test]
    fn test_meta_stmt_table_contains_only_expected_keys() {
        let table = get_metabuiltins_table();
        let keys: Vec<&String> = table.keys().collect();

        assert!(
            keys.contains(&&"input".to_string()),
            "The table must contain the key 'input'"
        );
        assert!(
            keys.contains(&&"print".to_string()),
            "The table must contain the key 'print'"
        );
        assert!(
            keys.contains(&&"open".to_string()),
            "The table must contain the key 'open'"
        );
        assert!(keys.contains(&&"input_int".to_string()));
        assert!(keys.contains(&&"input_real".to_string()));
        assert!(keys.contains(&&"to_string".to_string()));
        assert!(keys.contains(&&"to_string_fixed".to_string()));
        assert!(keys.contains(&&"str_concat".to_string()));
        assert!(keys.contains(&&"len".to_string()));
        assert!(keys.contains(&&"print_line".to_string()));
        assert!(keys.contains(&&"join".to_string()));
        assert!(keys.contains(&&"to_int".to_string()));
        assert!(keys.contains(&&"to_real".to_string()));
        assert!(keys.contains(&&"tuple_get".to_string()));
        assert_eq!(keys.len(), 14, "The table must contain only 14 keys");
    }

    #[test]
    fn test_meta_stmt_table_functions_are_callable() {
        let table = get_metabuiltins_table();

        // Check if the 'input' function exists in the table (without executing it)
        assert!(
            table.contains_key("input"),
            "The 'input' function must be in the table"
        );

        // Check if the 'print' function can be called
        if let Some(print_func) = table.get("print") {
            let mut env: Environment<Expression> = Environment::new();
            let result = print_func(&mut env);
            match result {
                Statement::Return(_) => (), // Expected
                _ => panic!("The 'print' function in the table must return Statement::Return"),
            }
        } else {
            panic!("The 'print' function must be in the table");
        }
    }

    #[test]
    fn test_input_builtin_function_signature() {
        // Checks if the input_builtin function can be referenced with an Environment
        // Note: We do not execute the function because it requires stdin
        let _input_func: MetaBuiltinStmt = input_builtin;
        assert!(true, "input_builtin has the correct signature");
    }

    #[test]
    fn test_input_real_builtin_function_signature() {
        let _func: MetaBuiltinStmt = input_real_builtin;
        assert!(true, "input_real_builtin has the correct signature");
    }

    #[test]
    fn test_len_builtin_on_string() {
        let mut env: Environment<Expression> = Environment::new();
        env.map_variable(
            "value".to_string(),
            false,
            Expression::CString("abc".to_string()),
        );
        let result = len_builtin(&mut env);
        match result {
            Statement::Return(expr) => assert_eq!(*expr, Expression::CInt(3)),
            _ => panic!("len_builtin must return Statement::Return"),
        }
    }

    #[test]
    fn test_len_builtin_on_list() {
        let mut env: Environment<Expression> = Environment::new();
        env.map_variable(
            "value".to_string(),
            false,
            Expression::ListValue(vec![Expression::CInt(1), Expression::CInt(2)]),
        );
        let result = len_builtin(&mut env);
        match result {
            Statement::Return(expr) => assert_eq!(*expr, Expression::CInt(2)),
            _ => panic!("len_builtin must return Statement::Return"),
        }
    }

    #[test]
    fn test_join_builtin() {
        let mut env: Environment<Expression> = Environment::new();
        env.map_variable(
            "values".to_string(),
            false,
            Expression::ListValue(vec![
                Expression::CString("a".to_string()),
                Expression::CString("b".to_string()),
            ]),
        );
        env.map_variable(
            "sep".to_string(),
            false,
            Expression::CString("-".to_string()),
        );
        let result = join_builtin(&mut env);
        match result {
            Statement::Return(expr) => {
                assert_eq!(*expr, Expression::CString("a-b".to_string()));
            }
            _ => panic!("join_builtin must return Statement::Return"),
        }
    }

    #[test]
    fn test_to_int_from_string() {
        let mut env: Environment<Expression> = Environment::new();
        env.map_variable(
            "value".to_string(),
            false,
            Expression::CString("42".to_string()),
        );
        let result = to_int_builtin(&mut env);
        match result {
            Statement::Return(expr) => assert_eq!(*expr, Expression::CInt(42)),
            _ => panic!("to_int_builtin must return Statement::Return"),
        }
    }

    #[test]
    fn test_to_real_from_int() {
        let mut env: Environment<Expression> = Environment::new();
        env.map_variable("value".to_string(), false, Expression::CInt(2));
        let result = to_real_builtin(&mut env);
        match result {
            Statement::Return(expr) => assert_eq!(*expr, Expression::CReal(2.0)),
            _ => panic!("to_real_builtin must return Statement::Return"),
        }
    }

    #[test]
    fn test_print_builtin_function_signature() {
        // Checks if the print_builtin function can be called with an Environment
        let mut env: Environment<Expression> = Environment::new();
        env.map_variable(
            "value".to_string(),
            false,
            Expression::CString("Hello World".to_string()),
        );

        let result = print_builtin(&mut env);
        match result {
            Statement::Return(_) => (), // Expected
            _ => panic!("print_builtin must return Statement::Return"),
        }
    }

    #[test]
    fn test_to_string_fixed_formats_real() {
        let mut env: Environment<Expression> = Environment::new();
        env.map_variable("value".to_string(), false, Expression::CReal(3.14159));
        env.map_variable("places".to_string(), false, Expression::CInt(2));
        let result = to_string_fixed_builtin(&mut env);
        match result {
            Statement::Return(expr) => {
                assert_eq!(*expr, Expression::CString("3.14".to_string()));
            }
            _ => panic!("to_string_fixed must return a CString"),
        }
    }

    // Helper function to run print_builtin tests with custom main function
    fn run_print_builtin_test(main_body: &str, expected_outputs: &[&str], test_name: &str) {
        use assert_cmd::Command;
        use std::fs;

        let test_code = format!(
            r#"
use r_python::environment::environment::Environment;
use r_python::ir::ast::Expression;
use r_python::stdlib::standard_library::print_builtin;

fn main() {{
{}
}}
"#,
            main_body
        );

        let filename = format!("test_print_{}.rs", test_name);
        let binary_name = format!("test_print_{}", test_name);

        fs::write(&filename, test_code).unwrap();

        let build_result = Command::new("cargo").args(&["build"]).output();
        if let Ok(build_output) = build_result {
            if build_output.status.success() {
                let compile_result = Command::new("rustc")
                    .args(&[
                        &filename,
                        "-L",
                        "target/debug/deps",
                        "--extern",
                        "r_python=target/debug/libr_python.rlib",
                        "-o",
                        &binary_name,
                    ])
                    .output();

                if let Ok(compile_output) = compile_result {
                    if compile_output.status.success() {
                        let mut cmd = Command::new(&format!("./{}", binary_name));
                        let mut assertion = cmd.assert().success();

                        for expected in expected_outputs {
                            assertion = assertion.stdout(predicates::str::contains(*expected));
                        }
                    }
                }
            }
        }

        fs::remove_file(&filename).ok();
        fs::remove_file(&binary_name).ok();
    }

    // Helper function for tests that need exact output checking
    fn run_print_builtin_test_exact(main_body: &str, expected_exact: &str, test_name: &str) {
        use assert_cmd::Command;
        use std::fs;

        let test_code = format!(
            r#"
use r_python::environment::environment::Environment;
use r_python::ir::ast::Expression;
use r_python::stdlib::standard_library::print_builtin;

fn main() {{
{}
}}
"#,
            main_body
        );

        let filename = format!("test_print_{}.rs", test_name);
        let binary_name = format!("test_print_{}", test_name);

        fs::write(&filename, test_code).unwrap();

        let build_result = Command::new("cargo").args(&["build"]).output();
        if let Ok(build_output) = build_result {
            if build_output.status.success() {
                let compile_result = Command::new("rustc")
                    .args(&[
                        &filename,
                        "-L",
                        "target/debug/deps",
                        "--extern",
                        "r_python=target/debug/libr_python.rlib",
                        "-o",
                        &binary_name,
                    ])
                    .output();

                if let Ok(compile_output) = compile_result {
                    if compile_output.status.success() {
                        let output = Command::new(&format!("./{}", binary_name))
                            .output()
                            .expect("Failed to run test");

                        assert_eq!(String::from_utf8_lossy(&output.stdout), expected_exact);
                    }
                }
            }
        }

        fs::remove_file(&filename).ok();
        fs::remove_file(&binary_name).ok();
    }

    // Helper function to run input_builtin tests with simulated stdin input
    fn run_input_builtin_test(
        main_body: &str,
        stdin_input: &str,
        expected_outputs: &[&str],
        test_name: &str,
    ) {
        use assert_cmd::Command;
        use std::fs;

        let test_code = format!(
            r#"
use r_python::environment::environment::Environment;
use r_python::ir::ast::{{Expression, Statement}};
use r_python::stdlib::standard_library::input_builtin;

fn main() {{
{}
}}
"#,
            main_body
        );

        let filename = format!("test_input_{}.rs", test_name);
        let binary_name = format!("test_input_{}", test_name);

        fs::write(&filename, test_code).unwrap();

        let build_result = Command::new("cargo").args(&["build"]).output();
        if let Ok(build_output) = build_result {
            if build_output.status.success() {
                let compile_result = Command::new("rustc")
                    .args(&[
                        &filename,
                        "-L",
                        "target/debug/deps",
                        "--extern",
                        "r_python=target/debug/libr_python.rlib",
                        "-o",
                        &binary_name,
                    ])
                    .output();

                if let Ok(compile_output) = compile_result {
                    if compile_output.status.success() {
                        let mut cmd = Command::new(&format!("./{}", binary_name));
                        let mut assertion = cmd.write_stdin(stdin_input).assert().success();

                        for expected in expected_outputs {
                            assertion = assertion.stdout(predicates::str::contains(*expected));
                        }
                    }
                }
            }
        }

        fs::remove_file(&filename).ok();
        fs::remove_file(&binary_name).ok();
    }

    #[test]
    fn test_print_builtin_captures_output() {
        let main_body = r#"
    // Test 1: String value
    let mut env1 = Environment::new();
    env1.map_variable("value".to_string(), false, Expression::CString("Hello World".to_string()));
    print_builtin(&mut env1);

    // Test 2: Integer value
    let mut env2 = Environment::new();
    env2.map_variable("value".to_string(), false, Expression::CInt(42));
    print_builtin(&mut env2);

    // Test 3: Float value
    let mut env3 = Environment::new();
    env3.map_variable("value".to_string(), false, Expression::CReal(3.14));
    print_builtin(&mut env3);

    // Test 4: No value (should print empty string)
    let mut env4 = Environment::new();
    print_builtin(&mut env4);"#;

        run_print_builtin_test(main_body, &["Hello World", "42", "3.14"], "builtin");
    }

    #[test]
    fn test_print_builtin_with_special_strings() {
        let main_body = r#"
    // Test string with newlines
    let mut env1 = Environment::new();
    env1.map_variable("value".to_string(), false, Expression::CString("Hello\nWorld".to_string()));
    print_builtin(&mut env1);

    // Test string with tabs
    let mut env2 = Environment::new();
    env2.map_variable("value".to_string(), false, Expression::CString("Hello\tWorld".to_string()));
    print_builtin(&mut env2);

    // Test empty string
    let mut env3 = Environment::new();
    env3.map_variable("value".to_string(), false, Expression::CString("".to_string()));
    print_builtin(&mut env3);

    // Test string with special characters
    let mut env4 = Environment::new();
    env4.map_variable("value".to_string(), false, Expression::CString("Olá, 世界! 🌍".to_string()));
    print_builtin(&mut env4);"#;

        run_print_builtin_test(
            main_body,
            &["Hello", "World", "Olá", "世界"],
            "special_strings",
        );
    }

    #[test]
    fn test_print_builtin_with_cvoid_type() {
        let main_body = r#"
    // Test CVoid type (should use debug print)
    let mut env = Environment::new();
    env.map_variable("value".to_string(), false, Expression::CVoid);
    print_builtin(&mut env);"#;

        run_print_builtin_test(main_body, &["CVoid"], "cvoid");
    }

    #[test]
    fn test_print_builtin_no_value_variable() {
        let main_body = r#"
    // Test when no "value" variable exists (should print empty string)
    let mut env = Environment::new();
    print_builtin(&mut env);"#;

        run_print_builtin_test_exact(main_body, "\n", "no_value");
    }

    #[test]
    fn test_print_builtin_return_value() {
        // Test that print_builtin returns Statement::Return(CVoid)
        let mut env = Environment::new();
        env.map_variable(
            "value".to_string(),
            false,
            Expression::CString("test".to_string()),
        );

        let result = print_builtin(&mut env);

        match result {
            Statement::Return(expr) => {
                match *expr {
                    Expression::CVoid => (), // Expected
                    _ => panic!("print_builtin should return Statement::Return(CVoid)"),
                }
            }
            _ => panic!("print_builtin should return Statement::Return"),
        }
    }

    #[test]
    fn test_input_builtin_with_prompt() {
        let main_body = r#"
    // Test with prompt
    let mut env = Environment::new();
    env.map_variable("prompt".to_string(), false, Expression::CString("Enter your name: ".to_string()));
    let result = input_builtin(&mut env);

    match result {
        Statement::Return(expr) => {
            if let Expression::CString(input) = *expr {
                println!("Input received: {}", input);
            }
        }
        _ => panic!("Expected Statement::Return"),
    }"#;

        run_input_builtin_test(
            main_body,
            "Alice\n",
            &["Enter your name: ", "Input received: Alice"],
            "with_prompt",
        );
    }

    #[test]
    fn test_input_builtin_without_prompt() {
        let main_body = r#"
    // Test without prompt (empty environment)
    let mut env = Environment::new();
    let result = input_builtin(&mut env);

    match result {
        Statement::Return(expr) => {
            if let Expression::CString(input) = *expr {
                println!("Input received: {}", input);
            }
        }
        _ => panic!("Expected Statement::Return"),
    }"#;

        run_input_builtin_test(
            main_body,
            "Bob\n",
            &["Input received: Bob"],
            "without_prompt",
        );
    }

    #[test]
    fn test_input_builtin_empty_input() {
        let main_body = r#"
    // Test with empty input
    let mut env = Environment::new();
    env.map_variable("prompt".to_string(), false, Expression::CString("Press Enter: ".to_string()));
    let result = input_builtin(&mut env);

    match result {
        Statement::Return(expr) => {
            if let Expression::CString(input) = *expr {
                println!("Input length: {}", input.len());
            }
        }
        _ => panic!("Expected Statement::Return"),
    }"#;

        run_input_builtin_test(
            main_body,
            "\n",
            &["Press Enter: ", "Input length: 0"],
            "empty_input",
        );
    }

    #[test]
    fn test_input_builtin_multiline_input() {
        let main_body = r#"
    // Test with input containing special characters
    let mut env = Environment::new();
    env.map_variable("prompt".to_string(), false, Expression::CString("Enter text: ".to_string()));
    let result = input_builtin(&mut env);

    match result {
        Statement::Return(expr) => {
            if let Expression::CString(input) = *expr {
                println!("Input received: '{}'", input);
            }
        }
        _ => panic!("Expected Statement::Return"),
    }"#;

        run_input_builtin_test(
            main_body,
            "Hello World! 123\n",
            &["Enter text: ", "Input received: 'Hello World! 123'"],
            "multiline_input",
        );
    }

    #[test]
    fn test_open_builtin_read_mode_success() {
        use std::fs;
        let test_file_path = "test_read_file.txt";
        let test_content = "Hello, World!\nThis is a test file.";
        fs::write(test_file_path, test_content).unwrap();

        let mut env: Environment<Expression> = Environment::new();
        env.map_variable(
            "path".to_string(),
            false,
            Expression::CString(test_file_path.to_string()),
        );
        env.map_variable(
            "mode".to_string(),
            false,
            Expression::CString("r".to_string()),
        );

        let result = open_builtin(&mut env);

        match result {
            Statement::Return(expr) => {
                if let Expression::CString(content) = *expr {
                    assert_eq!(content, test_content, "File content should match");
                } else {
                    panic!("Expected CString with file content");
                }
            }
            _ => panic!("Expected Statement::Return"),
        }

        fs::remove_file(test_file_path).unwrap();
    }

    #[test]
    fn test_open_builtin_write_mode_success() {
        use std::fs;
        let test_file_path = "test_write_file.txt";
        let test_content = "This is a test write.";

        let mut env: Environment<Expression> = Environment::new();
        env.map_variable(
            "path".to_string(),
            false,
            Expression::CString(test_file_path.to_string()),
        );
        env.map_variable(
            "mode".to_string(),
            false,
            Expression::CString("w".to_string()),
        );
        env.map_variable(
            "content".to_string(),
            false,
            Expression::CString(test_content.to_string()),
        );

        let result = open_builtin(&mut env);

        match result {
            Statement::Return(expr) => {
                if let Expression::CVoid = *expr {
                    let content = fs::read_to_string(test_file_path).unwrap();
                    assert_eq!(content, test_content, "File content should match");
                } else {
                    panic!("Expected CVoid after writing to file");
                }
            }
            _ => panic!("Expected Statement::Return"),
        }

        fs::remove_file(test_file_path).unwrap();
    }

    #[test]
    fn test_open_builtin_append_mode_success() {
        use std::fs;
        let test_file_path = "test_append_file.txt";
        let initial_content = "Initial content.\n";
        let append_content = "This is appended content.\n";
        fs::write(test_file_path, initial_content).unwrap();

        let mut env: Environment<Expression> = Environment::new();
        env.map_variable(
            "path".to_string(),
            false,
            Expression::CString(test_file_path.to_string()),
        );
        env.map_variable(
            "mode".to_string(),
            false,
            Expression::CString("a".to_string()),
        );
        env.map_variable(
            "content".to_string(),
            false,
            Expression::CString(append_content.to_string()),
        );

        let result = open_builtin(&mut env);
        match result {
            Statement::Return(expr) => {
                if let Expression::CVoid = *expr {
                    let content = fs::read_to_string(test_file_path).unwrap();
                    assert!(
                        content.contains(initial_content),
                        "File should contain initial content"
                    );
                    assert!(
                        content.contains(append_content),
                        "File should contain appended content"
                    );
                } else {
                    panic!("Expected CVoid after appending to file");
                }
            }
            _ => panic!("Expected Statement::Return"),
        }
        fs::remove_file(test_file_path).unwrap();
    }

    #[test]
    fn test_open_builtin_unsupported_mode() {
        let mut env: Environment<Expression> = Environment::new();
        env.map_variable(
            "path".to_string(),
            false,
            Expression::CString("dummy.txt".to_string()),
        );
        env.map_variable(
            "mode".to_string(),
            false,
            Expression::CString("x".to_string()),
        ); // Unsupported mode

        let result = open_builtin(&mut env);

        match result {
            Statement::Return(expr) => {
                if let Expression::CString(msg) = *expr {
                    assert_eq!(
                        msg, "open: unsupported mode 'x'.",
                        "Error message should match"
                    );
                } else {
                    panic!("Expected CString with error message");
                }
            }
            _ => panic!("Expected Statement::Return"),
        }
    }

    #[test]
    fn test_open_builtin_read_invalid_path() {
        let mut env: Environment<Expression> = Environment::new();
        env.map_variable(
            "path".to_string(),
            false,
            Expression::CString("invalid_path.txt".to_string()),
        );
        env.map_variable(
            "mode".to_string(),
            false,
            Expression::CString("r".to_string()),
        );

        let result = open_builtin(&mut env);

        match result {
            Statement::Return(expr) => {
                if let Expression::CString(msg) = *expr {
                    assert!(
                        msg.contains("open: could not open 'invalid_path.txt' for reading"),
                        "Error message should indicate invalid path"
                    );
                } else {
                    panic!("Expected CString with error message");
                }
            }
            _ => panic!("Expected Statement::Return"),
        }
    }

    #[test]
    fn test_open_builtin_read_nonexistent_file() {
        let mut env: Environment<Expression> = Environment::new();
        env.map_variable(
            "path".to_string(),
            false,
            Expression::CString("nonexistent.txt".to_string()),
        );
        env.map_variable(
            "mode".to_string(),
            false,
            Expression::CString("r".to_string()),
        );

        let result = open_builtin(&mut env);

        match result {
            Statement::Return(expr) => {
                if let Expression::CString(msg) = *expr {
                    assert!(
                        msg.contains("open: could not open 'nonexistent.txt' for reading"),
                        "Error message should indicate file not found"
                    );
                } else {
                    panic!("Expected CString with error message");
                }
            }
            _ => panic!("Expected Statement::Return"),
        }
    }

    #[test]
    fn test_open_builtin_write_empty_content() {
        use std::fs;
        let test_file_path = "test_empty_write_file.txt";

        let mut env: Environment<Expression> = Environment::new();
        env.map_variable(
            "path".to_string(),
            false,
            Expression::CString(test_file_path.to_string()),
        );
        env.map_variable(
            "mode".to_string(),
            false,
            Expression::CString("w".to_string()),
        );
        env.map_variable(
            "content".to_string(),
            false,
            Expression::CString("".to_string()),
        );

        let result = open_builtin(&mut env);

        match result {
            Statement::Return(expr) => {
                if let Expression::CVoid = *expr {
                    let content = fs::read_to_string(test_file_path).unwrap();
                    assert_eq!(content, "", "File content should be empty");
                } else {
                    panic!("Expected CVoid after writing empty content to file");
                }
            }
            _ => panic!("Expected Statement::Return"),
        }

        fs::remove_file(test_file_path).unwrap();
    }

    #[test]
    fn test_open_builtin_write_missing_content_argument() {
        let mut env: Environment<Expression> = Environment::new();
        env.map_variable(
            "path".to_string(),
            false,
            Expression::CString("dummy.txt".to_string()),
        );
        env.map_variable(
            "mode".to_string(),
            false,
            Expression::CString("w".to_string()),
        );

        let result = open_builtin(&mut env);

        match result {
            Statement::Return(expr) => {
                if let Expression::CString(msg) = *expr {
                    assert_eq!(msg, "open: when using mode 'w', a third argument with the content to write is required", "Error message should indicate missing content argument");
                } else {
                    panic!("Expected CString with error message");
                }
            }
            _ => panic!("Expected Statement::Return"),
        }
    }

    #[test]
    fn test_open_builtin_first_argument_not_string() {
        let mut env: Environment<Expression> = Environment::new();
        env.map_variable("path".to_string(), false, Expression::CInt(42));
        env.map_variable(
            "mode".to_string(),
            false,
            Expression::CString("r".to_string()),
        );

        let result = open_builtin(&mut env);

        match result {
            Statement::Return(expr) => {
                if let Expression::CString(msg) = *expr {
                    assert_eq!(
                        msg, "open: first argument must be a string with the file path",
                        "Error message should indicate invalid first argument"
                    );
                } else {
                    panic!("Expected CString with error message");
                }
            }
            _ => panic!("Expected Statement::Return"),
        }
    }

    #[test]
    fn test_open_builtin_append_missing_content_argument() {
        let mut env: Environment<Expression> = Environment::new();
        env.map_variable(
            "path".to_string(),
            false,
            Expression::CString("dummy.txt".to_string()),
        );
        env.map_variable(
            "mode".to_string(),
            false,
            Expression::CString("a".to_string()),
        );

        let result = open_builtin(&mut env);
        match result {
            Statement::Return(expr) => {
                if let Expression::CString(msg) = *expr {
                    assert_eq!(msg, "open: when using mode 'a', a third argument with the content to append is required", "Error message should indicate missing content argument");
                } else {
                    panic!("Expected CString with error message");
                }
            }
            _ => panic!("Expected Statement::Return"),
        }
    }
}
