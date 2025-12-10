use egglog::{
    EGraph, Error,
    ast::{Expr, Literal, RustSpan, Span},
    call,
    prelude::exprs::{float, int},
};

pub(crate) fn add_to_egraph(eg: &mut EGraph) -> Result<(), Error> {
    eg.parse_and_run_program(Some("f-smooth.egg".into()), include_str!("f-smooth.egg"))?;
    Ok(())
}

pub fn lam(nargs: u32, mut body: Expr) -> Expr {
    for _ in 0..nargs {
        body = call!("Lam", [body]);
    }
    body
}

pub fn app(mut fun: Expr, args: impl IntoIterator<Item = Expr>) -> Expr {
    for arg in args {
        fun = call!("App", [fun, arg]);
    }
    fun
}

pub fn var(n: u32) -> Expr {
    call!("Var", [int(n.into())])
}

pub fn app_prim(op: &str, args: impl IntoIterator<Item = Expr>) -> Expr {
    app(call!("Prim", [call!(op, [])]), args)
}

pub fn real(x: f64) -> Expr {
    call!("Real", [float(x)])
}

pub fn inte(n: i64) -> Expr {
    call!("Int", [int(n)])
}

pub fn shift(cutoff: i64, amount: i64, expr: &mut Expr) {
    if let Expr::Call(_, head, tail) = expr {
        match (head.as_str(), tail.as_mut_slice()) {
            ("Var", [Expr::Lit(_, Literal::Int(n))]) if *n >= cutoff => *n += amount,
            ("Lam", [body]) => shift(cutoff + 1, amount, body),
            ("App", [fun, arg]) => {
                shift(cutoff, amount, fun);
                shift(cutoff, amount, arg);
            }
            _ => {}
        }
    }
}
