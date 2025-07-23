use std::collections::HashMap;
use std::sync::Arc;
use crate::EvalError;

pub type FunctionImpl = Arc<dyn for<'a> Fn(&'a [f64]) -> Result<f64, EvalError> + Send + Sync>;

pub struct Context {
    vars: HashMap<String, f64>,
    funcs: HashMap<String, FunctionImpl>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            funcs: HashMap::new(),
        }
    }

    pub fn with_var(mut self, name: &str, val: f64) -> Self {
        self.vars.insert(name.to_string(), val);
        self
    }

    pub fn with_func<F>(mut self, name: &str, func: F) -> Self
    where
        F: for<'a> Fn(&'a [f64]) -> Result<f64, EvalError> + Send + Sync + 'static,
    {
        self.funcs.insert(name.to_string(), Arc::new(func));
        self
    }

    pub fn get_var(&self, name: &str) -> Option<f64> {
        self.vars.get(name).copied()
    }

    pub fn get_func(&self, name: &str) -> Option<&FunctionImpl> {
        self.funcs.get(name)
    }
}
