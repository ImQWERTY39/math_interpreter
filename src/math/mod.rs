use crate::identifier::{EvaluationResult, Name, VariableScope};
use crate::tokeniser::Token;

#[derive(Debug)]
pub enum Operation {
    Number(f64),
    Variable(Name),
    Negate(Box<Operation>),
    Add(Box<Operation>, Box<Operation>),
    Subtract(Box<Operation>, Box<Operation>),
    Multiply(Box<Operation>, Box<Operation>),
    Divide(Box<Operation>, Box<Operation>),
    Exponentiate(Box<Operation>, Box<Operation>),
}

impl Operation {
    pub fn evaluate(&self, var_scop: &VariableScope) -> EvaluationResult {
        match self {
            Operation::Number(i) => Ok(*i),
            Operation::Variable(i) => var_scop.get(i).ok_or("Variable doesnt exist").copied(),
            Operation::Negate(i) => i.evaluate(var_scop).map(|x| -x),
            Operation::Add(i, j) => Ok(i.evaluate(var_scop)? + j.evaluate(var_scop)?),
            Operation::Subtract(i, j) => Ok(i.evaluate(var_scop)? - j.evaluate(var_scop)?),
            Operation::Multiply(i, j) => Ok(i.evaluate(var_scop)? * j.evaluate(var_scop)?),
            Operation::Divide(i, j) => Ok(i.evaluate(var_scop)? / j.evaluate(var_scop)?),
            Operation::Exponentiate(i, j) => Ok(i.evaluate(var_scop)?.powf(j.evaluate(var_scop)?)),
        }
    }
}

#[derive(Debug)]
pub enum ExpressionResult {
    Evaluate(Operation),
    AssignVariable(Name, Operation),
}

pub fn generate_ast(mut tokens: Vec<Token>) -> Result<ExpressionResult, &'static str> {
    let equals_index: Vec<usize> = tokens
        .iter()
        .enumerate()
        .filter_map(|(i, x)| {
            if matches!(*x, Token::Equals) {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    if equals_index.len() > 1 {
        return Err("Can't have more than 1 equals symbol in an expression");
    }

    if equals_index.len() == 1 {
        if !matches!(tokens[1], Token::Equals) {
            return Err("Must have only a single variable name before the equals symbol");
        }

        let var_name = tokens.drain(..1).next();
        let variable_name = if let Some(Token::Variable(i)) = var_name {
            i
        } else {
            return Err("Can have only  variable name before the equals symbol");
        };

        return Ok(ExpressionResult::AssignVariable(
            variable_name,
            make_ast(&tokens[1..])?,
        ));
    }

    Ok(ExpressionResult::Evaluate(make_ast(&tokens)?))
}

fn make_ast(tokens: &[Token]) -> Result<Operation, &'static str> {
    if wrapped_in_brackets(tokens) {
        return make_ast(&tokens[1..tokens.len() - 1]);
    }

    let idx = get_lowest_precedence(tokens);
    match tokens[idx] {
        Token::Number(i) => Ok(Operation::Number(i)),
        Token::Variable(ref i) => Ok(Operation::Variable(i.clone())),
        Token::Plus => Ok(Operation::Add(
            Box::new(make_ast(&tokens[..idx])?),
            Box::new(make_ast(&tokens[idx + 1..])?),
        )),
        Token::Minus => {
            if idx == 0 {
                Ok(Operation::Negate(Box::new(make_ast(&tokens[1..])?)))
            } else {
                Ok(Operation::Subtract(
                    Box::new(make_ast(&tokens[..idx])?),
                    Box::new(make_ast(&tokens[idx + 1..])?),
                ))
            }
        }
        Token::Star => Ok(Operation::Multiply(
            Box::new(make_ast(&tokens[..idx])?),
            Box::new(make_ast(&tokens[idx + 1..])?),
        )),
        Token::Slash => Ok(Operation::Divide(
            Box::new(make_ast(&tokens[..idx])?),
            Box::new(make_ast(&tokens[idx + 1..])?),
        )),
        Token::Exponent => Ok(Operation::Exponentiate(
            Box::new(make_ast(&tokens[..idx])?),
            Box::new(make_ast(&tokens[idx + 1..])?),
        )),
        Token::Percent => todo!(),
        _ => unreachable!(),
    }
}

fn get_lowest_precedence(tokens: &[Token]) -> usize {
    let mut lowest_idx = 0;
    let mut lowest_rank = tokens[0].rank();
    let mut count = 0;

    for (i, v) in tokens.iter().enumerate() {
        match v {
            Token::OpenBracket => count += 1,
            Token::CloseBracket => count -= 1,
            _ => (),
        };

        if v.rank() < lowest_rank && count == 0 {
            lowest_idx = i;
            lowest_rank = v.rank();
        }
    }

    lowest_idx
}

fn wrapped_in_brackets(tokens: &[Token]) -> bool {
    let start_width_bracket = tokens
        .first()
        .is_some_and(|x| matches!(*x, Token::OpenBracket));
    let ends_width_bracket = tokens
        .last()
        .is_some_and(|x| matches!(*x, Token::CloseBracket));

    if !(start_width_bracket && ends_width_bracket) {
        return false;
    }

    let mut count = 1;
    for i in &tokens[1..tokens.len() - 1] {
        match i {
            Token::OpenBracket => count += 1,
            Token::CloseBracket => count -= 1,
            _ => (),
        };

        if count == 0 {
            return false;
        }
    }

    true
}
