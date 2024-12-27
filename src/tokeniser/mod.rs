use crate::identifier::Name;

#[derive(Debug)]
pub enum Token {
    Number(f64),
    Variable(Name),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Exponent,
    Equals,
    OpenBracket,
    CloseBracket,
}

impl Token {
    pub fn rank(&self) -> u8 {
        match self {
            Token::Number(_) => 4,
            Token::Variable(_) => 4,
            Token::Plus => 1,
            Token::Minus => 1,
            Token::Star => 2,
            Token::Slash => 2,
            Token::Exponent => 3,
            Token::Percent => 7,
            Token::Equals => 7,
            Token::OpenBracket => 7,
            Token::CloseBracket => 7,
        }
    }
}

impl TryFrom<&str> for Token {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(i) = value.parse::<f64>() {
            return Ok(Self::Number(i));
        }

        if valid_identifier(value) {
            return Ok(Self::Variable(value.into()));
        }

        match value {
            "+" => Ok(Token::Plus),
            "-" => Ok(Token::Minus),
            "*" => Ok(Token::Star),
            "/" => Ok(Token::Slash),
            "%" => Ok(Token::Percent),
            "^" => Ok(Token::Exponent),
            "=" => Ok(Token::Equals),
            "(" => Ok(Token::OpenBracket),
            ")" => Ok(Token::CloseBracket),
            _ => Err("unrecognised token"),
        }
    }
}

fn valid_identifier(token: &str) -> bool {
    let mut valid_identifiers: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
    valid_identifiers.push('_');

    if !valid_identifiers.contains(&token.chars().next().unwrap_or_default()) {
        return false;
    }

    valid_identifiers.extend('0'..='9');
    valid_identifiers.push('.');

    for i in token.chars() {
        if !valid_identifiers.contains(&i) {
            return false;
        }
    }

    true
}

pub fn tokenise(expr: &str) -> Result<Vec<Token>, &'static str> {
    let mut tokens = Vec::new();
    let mut builder = String::new();

    for i in expr.chars() {
        if i.is_whitespace() {
            if let Ok(i) = Token::try_from(builder.as_str()) {
                tokens.push(i);
                builder.clear();
            }

            continue;
        }

        builder.push(i);

        match Token::try_from(builder.as_str()) {
            Err(_) if !builder.is_empty() => {
                builder.pop();
                tokens.push(Token::try_from(builder.as_str())?);
                builder.clear();
                builder.push(i);
            }
            _ => (),
        }

        if builder == "-" {
            builder.clear();
            tokens.push(Token::Minus);
        }
    }

    if let Ok(tkn) = Token::try_from(builder.as_str()) {
        tokens.push(tkn);
    } else if !builder.is_empty() {
        return Err("Invalid expression");
    }

    Ok(correct_minus(tokens))
}

fn correct_minus(mut tokens: Vec<Token>) -> Vec<Token> {
    let mut i = 0;
    while i < tokens.len() {
        let is_minus = matches!(tokens[i], Token::Minus);
        let is_prev_not_operand = i == 0
            || tokens
                .get(i - 1)
                .is_some_and(|x| !matches!(x, Token::Number(_)) || matches!(x, Token::Variable(_)));
        let is_next_num = tokens
            .get(i + 1)
            .is_some_and(|x| matches!(x, Token::Number(_)));

        if is_minus && is_prev_not_operand && is_next_num {
            if let Token::Number(i) = tokens.get_mut(i + 1).expect("cannot fail") {
                *i *= -1.0;
            }
            tokens.remove(i);

            continue;
        }

        i += 1;
    }

    tokens
}
