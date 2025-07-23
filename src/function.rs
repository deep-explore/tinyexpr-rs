use crate::eval::EvalError;

/// Represents a user-defined or built-in function.
pub type FunctionImpl = Box<dyn Fn(&[f64]) -> Result<f64, EvalError> + Send + Sync>;
