use egglog::ast::{Expr, Literal, Span};

pub fn apply(span: Span, fun: impl Into<String>, args: Vec<Expr>) -> Expr {
    let fun_name = Expr::Lit(span.clone(), Literal::String(fun.into()));
    let fun_var = Expr::Call(span.clone(), "Var".into(), vec![fun_name]);
    let arg_vec = Expr::Call(span.clone(), "vec-of".into(), args);
    Expr::Call(span, "App".into(), vec![fun_var, arg_vec])
}

pub fn constant(span: Span, x: f64) -> Expr {
    let lit = Literal::Float(x.into());
    Expr::Lit(span, lit)
}
