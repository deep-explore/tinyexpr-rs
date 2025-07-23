//! # tinyexpr-rs
//!
//! A tiny expression evaluator for math expressions with variables, constants,
//! and support for functional evaluation via a context object.
//!
//! ## Example
//! ```rust
//! use tinyexpr_rs::*;
//!
//! let expr = "x + 2".parse::<Expr>().unwrap();
//! let ctx = Context::new().with_var("x", 3.0);
//! let result = expr.eval(&ctx).unwrap();
//! assert_eq!(result, 5.0);
//! ```

mod expr;
mod token;
mod ast;
mod context;


pub use token::{Lexer, Token, LexError};
pub use expr::{Expr, EvalError};
pub use ast::{Parser, ExprNode, ParseError};
pub use context::{Context, FunctionImpl};


use std::str::FromStr;

impl std::str::FromStr for ExprNode {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(s)?;
        parser.parse()
    }
}
