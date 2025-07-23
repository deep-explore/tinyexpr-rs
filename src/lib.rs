//! # tinyexpr-rs
//!
//! A tiny expression evaluator for math expressions with variables, constants,
//! and support for functional evaluation via a context object.
//!
//! ## Example
//! ```rust
//! use tinyexpr_rs::*;
//!
//! let expr = "x * x + 1".parse::<Expr>().unwrap();
//! let ctx = Context::new().with_var("x", 3.0);
//! let result = expr.eval(&ctx).unwrap();
//! assert_eq!(result, 10.0);
//! ```

mod expr;

pub use expr::{Context, EvalError, Expr};
