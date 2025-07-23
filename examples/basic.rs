use tinyexpr_rs::*;

fn main() {
    let expr = "x * x + 1".parse::<Expr>().unwrap();
    let ctx = Context::new().with_var("x", 3.0);
    println!("Result: {}", expr.eval(&ctx).unwrap()); // 10
}
