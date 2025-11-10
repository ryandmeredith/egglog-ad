use egglog::ast::{Expr, Literal, Macro, ParseError, Parser, Sexp, Span};
use std::sync::Arc;

struct Builtin {
    name: &'static str,
}

impl Builtin {
    fn new(name: &'static str) -> Arc<Self> {
        Arc::new(Self { name })
    }
}

impl Macro<Expr> for Builtin {
    fn name(&self) -> &str {
        self.name
    }

    fn parse(&self, args: &[Sexp], span: Span, parser: &mut Parser) -> Result<Expr, ParseError> {
        let fun_name = Expr::Lit(span.clone(), Literal::String(self.name.into()));
        let fun_var = Expr::Call(span.clone(), "Var".into(), vec![fun_name]);
        let arg_exprs = Result::from_iter(args.iter().map(|x| parser.parse_expr(x)))?;
        let arg_vec = Expr::Call(span.clone(), "vec-of".into(), arg_exprs);
        Ok(Expr::Call(span, "App".into(), vec![fun_var, arg_vec]))
    }
}

pub fn add_builtins(parser: &mut Parser) {
    // Scalar operators
    parser.add_expr_macro(Builtin::new("add"));
    parser.add_expr_macro(Builtin::new("sub"));
    parser.add_expr_macro(Builtin::new("mul"));
    parser.add_expr_macro(Builtin::new("div"));
    parser.add_expr_macro(Builtin::new("pow"));

    // Scalar functions
    parser.add_expr_macro(Builtin::new("exp"));
    parser.add_expr_macro(Builtin::new("log"));
    parser.add_expr_macro(Builtin::new("sin"));
    parser.add_expr_macro(Builtin::new("cos"));
    parser.add_expr_macro(Builtin::new("tan"));

    // Comparison operators
    parser.add_expr_macro(Builtin::new("lt"));
    parser.add_expr_macro(Builtin::new("gt"));
    parser.add_expr_macro(Builtin::new("eq"));

    // Boolean functions
    parser.add_expr_macro(Builtin::new("and"));
    parser.add_expr_macro(Builtin::new("or"));
    parser.add_expr_macro(Builtin::new("not"));

    // Array functions
    parser.add_expr_macro(Builtin::new("build"));
    parser.add_expr_macro(Builtin::new("ifold"));
    parser.add_expr_macro(Builtin::new("get"));
    parser.add_expr_macro(Builtin::new("length"));

    // Pair functions
    parser.add_expr_macro(Builtin::new("pair"));
    parser.add_expr_macro(Builtin::new("fst"));
    parser.add_expr_macro(Builtin::new("snd"));
}
