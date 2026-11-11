//! Type checks a RPython program without running it.
//!
//! Usage: cargo run --example typecheck -- <file.rpy>
//!
//! The interpreter does not execute classes yet, so this runner shows what the
//! type checker accepts or rejects for a program that uses them.

use r_python::environment::environment::Environment;
use r_python::ir::ast::Type;
use r_python::parser::parse;
use r_python::type_checker::check_stmt;
use std::process::exit;

fn main() {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("usage: cargo run --example typecheck -- <file.rpy>");
        exit(2);
    };
    let source = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("cannot read {path}: {e}");
        exit(2);
    });

    let stmts = match parse(&source) {
        Ok((rest, stmts)) if rest.trim().is_empty() => stmts,
        Ok((rest, _)) => {
            eprintln!("parse error: could not parse '{}'", rest.trim());
            exit(1);
        }
        Err(e) => {
            eprintln!("parse error: {e}");
            exit(1);
        }
    };

    let mut env: Environment<Type> = Environment::new();
    for stmt in stmts {
        match check_stmt(stmt, &env) {
            Ok(next) => env = next,
            Err(e) => {
                eprintln!("{e}");
                exit(1);
            }
        }
    }
    println!("{path}: ok, the program is well typed");
}
