use egglog::{
    EGraph, Error,
    ast::{Expr, Literal, RustSpan, Span},
    call,
    prelude::exprs::{float, int},
};

fn add_to_egraph(eg: &mut EGraph) -> Result<(), Error> {
    eg.parse_and_run_program(Some("f-smooth.egg".into()), include_str!("f-smooth.egg"))?;
    Ok(())
}

pub fn lam(nargs: u32, body: Expr) -> Expr {
    call!("Lam", [int(nargs.into()), body])
}

pub fn app(fun: Expr, args: impl IntoIterator<Item = Expr>) -> Expr {
    call!("App", [fun, call!("vec-of", args)])
}

pub fn var(n: i64) -> Expr {
    call!("Var", [int(n)])
}

pub fn app_prim(op: &str, args: impl IntoIterator<Item = Expr>) -> Expr {
    app(call!("Prim", [call!(op, [])]), args)
}

pub fn real(x: f64) -> Expr {
    call!("Prim", [call!("Real", [float(x)])])
}

pub fn inte(n: i64) -> Expr {
    call!("Prim", [call!("Int", [int(n)])])
}

pub fn shift(cutoff: i64, amount: i64, expr: &mut Expr) {
    if let Expr::Call(_, head, tail) = expr {
        match (head.as_str(), tail.as_mut_slice()) {
            ("Var", [Expr::Lit(_, Literal::Int(n))]) if *n >= cutoff => *n += amount,
            ("Lam", [Expr::Lit(_, Literal::Int(n)), body]) => shift(cutoff + *n, amount, body),
            ("App", [fun, Expr::Call(_, _, args)]) => {
                shift(cutoff, amount, fun);
                for arg in args {
                    shift(cutoff, amount, arg);
                }
            }
            _ => {}
        }
    }
}
