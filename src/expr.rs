use std::collections::HashMap;

#[derive(Debug)]
pub struct Context {
    vars: HashMap<String, f64>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn with_var(mut self, name: &str, val: f64) -> Self {
        self.vars.insert(name.to_string(), val);
        self
    }

    pub fn get(&self, name: &str) -> Option<f64> {
        self.vars.get(name).copied()
    }
}

#[derive(Debug)]
pub struct Expr {
    raw: String,
}

#[derive(Debug)]
pub enum EvalError {
    UnknownVariable(String),
    ParseError,
}

impl std::str::FromStr for Expr {
    type Err = EvalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { raw: s.to_string() })
    }
}

impl Expr {
    pub fn eval(&self, ctx: &Context) -> Result<f64, EvalError> {
        // Very basic parser for demo purposes only
        let tokens: Vec<&str> = self.raw.split_whitespace().collect();

        // Only handles: "<var> * <var> + <const>"
        if tokens.len() != 5 || tokens[1] != "*" || tokens[3] != "+" {
            return Err(EvalError::ParseError);
        }

        let a = ctx.get(tokens[0]).ok_or_else(|| EvalError::UnknownVariable(tokens[0].into()))?;
        let b = ctx.get(tokens[2]).ok_or_else(|| EvalError::UnknownVariable(tokens[2].into()))?;
        let c: f64 = tokens[4].parse().map_err(|_| EvalError::ParseError)?;

        Ok(a * b + c)
    }
}
