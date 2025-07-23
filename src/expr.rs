use crate::ast::{ExprNode, Parser, ParseError};
use std::collections::HashMap;
use std::str::FromStr;

/// A parsed expression. Use `.eval(&Context)` to evaluate it.
#[derive(Debug, Clone)]
pub struct Expr {
    inner: ExprNode,
}

impl Expr {
    /// Evaluate the expression using the given context.
    pub fn eval(&self, ctx: &Context) -> Result<f64, EvalError> {
        self.inner.eval(ctx)
    }
}

impl FromStr for Expr {
    type Err = ParseError;

    /// Parse an expression from a string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(s)?;
        let node = parser.parse()?;
        Ok(Expr { inner: node })
    }
}

/// Context holds variable bindings used during evaluation.
#[derive(Debug, Clone)]
pub struct Context {
    vars: HashMap<String, f64>,
}

impl Context {
    /// Create a new, empty context.
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    /// Add a variable to the context.
    pub fn with_var(mut self, name: &str, val: f64) -> Self {
        self.vars.insert(name.to_string(), val);
        self
    }

    /// Get a variable value by name.
    pub fn get(&self, name: &str) -> Option<f64> {
        self.vars.get(name).copied()
    }
}

/// Errors that can occur during evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    UnknownVariable(String),
    DivisionByZero,
    InvalidOperation,
}

impl ExprNode {
    /// Evaluate the AST node using the given context.
    pub fn eval(&self, ctx: &Context) -> Result<f64, EvalError> {
        match self {
            ExprNode::Number(n) => Ok(*n),
            ExprNode::Variable(name) => ctx
                .get(name)
                .ok_or_else(|| EvalError::UnknownVariable(name.clone())),
            ExprNode::BinaryOp { op, left, right } => {
                let l = left.eval(ctx)?;
                let r = right.eval(ctx)?;
                match op {
                    '+' => Ok(l + r),
                    '-' => Ok(l - r),
                    '*' => Ok(l * r),
                    '/' => {
                        if r == 0.0 {
                            Err(EvalError::DivisionByZero)
                        } else {
                            Ok(l / r)
                        }
                    }
                    '^' => Ok(l.powf(r)),
                    _ => Err(EvalError::InvalidOperation),
                }
            }
            ExprNode::FunctionCall { name, args } => {
                let arg_values: Result<Vec<f64>, EvalError> =
                    args.iter().map(|arg| arg.eval(ctx)).collect();
                let arg_values = arg_values?;

                match (name.as_str(), arg_values.as_slice()) {
                    ("sin", [x]) => Ok(x.sin()),
                    ("cos", [x]) => Ok(x.cos()),
                    ("sqrt", [x]) => Ok(x.sqrt()),
                    ("abs", [x]) => Ok(x.abs()),
                    ("max", [x, y]) => Ok(x.max(*y)),
                    ("min", [x, y]) => Ok(x.min(*y)),
                    _ => Err(EvalError::InvalidOperation),
                }
            }
        }
    }
}
