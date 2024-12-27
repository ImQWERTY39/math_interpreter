use std::collections::HashMap;

pub type EvaluationResult = Result<f64, &'static str>;
pub type Name = Box<str>;
pub type VariableScope = HashMap<Name, f64>;
