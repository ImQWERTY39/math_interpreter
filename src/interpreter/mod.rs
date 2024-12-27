use std::io::Write;

use crate::identifier::VariableScope;
use crate::math::{self, ExpressionResult};
use crate::tokeniser;

pub fn run() {
    let mut scope = init_scope();

    loop {
        let expr = input("> ");

        if expr.is_empty() {
            break;
        }

        let tokens = match tokeniser::tokenise(&expr) {
            Ok(i) => i,
            Err(i) => {
                eprintln!("{}\n", i);
                continue;
            }
        };

        let ast = math::generate_ast(tokens);

        match ast {
            Ok(ExpressionResult::Evaluate(i)) => show_result(i, &scope),
            Ok(ExpressionResult::AssignVariable(var, expr)) => assign_var(&mut scope, var, expr),
            Err(i) => eprintln!("{}\n", i),
        }
    }
}

fn init_scope() -> VariableScope {
    let mut scope = VariableScope::new();

    scope.insert("e".into(), std::f64::consts::E);
    scope.insert("pi".into(), std::f64::consts::PI);
    scope.insert("tau".into(), std::f64::consts::TAU);
    scope.insert("e".into(), std::f64::consts::E);
    scope.insert("e".into(), std::f64::consts::E);

    scope
}

fn show_result(expr: math::Operation, scope: &VariableScope) {
    match expr.evaluate(scope) {
        Ok(i) => println!("{i}"),
        Err(i) => eprintln!("{i}\n"),
    }
}

fn assign_var(scope: &mut VariableScope, var: Box<str>, expr: math::Operation) {
    let val = match expr.evaluate(scope) {
        Ok(i) => i,
        Err(i) => {
            eprintln!("{i}\n");
            return;
        }
    };

    scope.insert(var, val);
}

fn input(msg: &str) -> String {
    print!("{msg}");
    std::io::stdout().flush().expect("shouldn't have failed");

    let mut string = String::new();
    std::io::stdin()
        .read_line(&mut string)
        .expect("shouldn't have failed");

    string
}
