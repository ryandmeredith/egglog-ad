use crate::utils::{apply, constant};
use egglog::ast::{Expr, Literal};

fn forward(expr: Expr) -> Expr {
    match expr {
        Expr::Call(span, head, args) if head == "Const" => {
            let x = Expr::Call(span.clone(), head, args);
            let z = constant(span.clone(), 0.);
            apply(span, "pair", vec![x, z])
        }
        Expr::Call(span, head, mut args) if head == "Var" => {
            if let Expr::Lit(_, Literal::String(var)) = &mut args[0] {
                // Check for builtin here
                var.push('d')
            }
            Expr::Call(span, head, args)
        }
        x => x,
    }
}
