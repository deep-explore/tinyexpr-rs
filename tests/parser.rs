use tinyexpr_rs::*;
use std::str::FromStr;

#[test]
fn test_parse_arithmetic() {
    let expr = Expr::from_str("1 + 2 * 3").unwrap();
    let ctx = Context::new();
    let result = expr.eval(&ctx).unwrap();
    assert_eq!(result, 7.0);
}

#[test]
fn test_parse_with_variables() {
    let expr = Expr::from_str("x * 2").unwrap();
    let ctx = Context::new().with_var("x", 4.0);
    let result = expr.eval(&ctx).unwrap();
    assert_eq!(result, 8.0);
}

#[test]
fn test_function_call() {
    let expr = Expr::from_str("sqrt(16)").unwrap();
    let ctx = Context::new();
    let result = expr.eval(&ctx).unwrap();
    assert_eq!(result, 4.0);
}


#[test]
fn test_undefined_variable() {
    let expr = Expr::from_str("x + 1").unwrap();
    let ctx = Context::new();
    let result = expr.eval(&ctx);
    assert!(result.is_err());
}

#[test]
fn test_invalid_expression() {
    let expr = Expr::from_str("1 + * 2");
    assert!(expr.is_err());
}
