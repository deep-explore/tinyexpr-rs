use tinyexpr_rs::*;

fn main() {
    let expr = "sin(x) + sqrt(y)".parse::<Expr>().unwrap();
    let ctx = Context::new()
        .with_var("x", std::f64::consts::PI / 2.0)
        .with_var("y", 9.0);
    let result = expr.eval(&ctx).unwrap();
    println!("Result: {}", result); // Should print 1 + 3 = 4
}
