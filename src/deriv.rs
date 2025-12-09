use egglog::{
    EGraph, Error, RunReport,
    ast::{Command, Expr, Rewrite, RustSpan, Span},
    call, span,
};

use crate::{
    dsl::{Arg, D},
    f_smooth::{self, app},
};

pub fn d(expr: Expr) -> Expr {
    call!("D", [expr])
}

fn case(op: &str, deriv: D) -> Command {
    let rewrite = Rewrite {
        span: span!(),
        lhs: call!("P", [call!(op, [])]),
        rhs: deriv.0,
        conditions: vec![],
    };
    Command::Rewrite("deriv".into(), rewrite, false)
}

pub(crate) fn add_to_egraph(eg: &mut EGraph) -> Result<(), Error> {
    f_smooth::add_to_egraph(eg)?;
    eg.parse_and_run_program(Some("deriv.egg".into()), include_str!("deriv.egg"))?;
    eg.run_program(vec![
        case(
            "Add",
            D::fun2(|x, y| D::pair(x.fst() + y.fst(), x.snd() + y.snd())),
        ),
        case(
            "Sub",
            D::fun2(|x, y| D::pair(x.fst() - y.fst(), x.snd() - y.snd())),
        ),
        case(
            "Mul",
            D::fun2(|x, y| D::pair(x.fst() * y.fst(), x.snd() * y.fst() + x.fst() * y.snd())),
        ),
        case(
            "Div",
            D::fun2(|x, y| {
                D::pair(
                    x.fst() / y.fst(),
                    (x.snd() * y.fst() - x.fst() * y.snd()) / y.pow(2.),
                )
            }),
        ),
        case(
            "Pow",
            D::fun2(|x, y| {
                D::pair(
                    x.fst().pow(y.fst()),
                    (y.fst() * x.snd() / x.fst() + x.fst().log() * y.snd()) * x.fst().pow(y.fst()),
                )
            }),
        ),
        case(
            "Exp",
            D::fun(|x| D::pair(x.fst().exp(), x.snd() * x.fst().exp())),
        ),
        case("Log", D::fun(|x| D::pair(x.fst().exp(), x.snd() / x.fst()))),
        case(
            "Sin",
            D::fun(|x| D::pair(x.fst().sin(), x.snd() * x.fst().cos())),
        ),
        case(
            "Cos",
            D::fun(|x| D::pair(x.fst().cos(), -x.snd() * x.fst().sin())),
        ),
        case("LT", D::fun2(|x, y| x.fst().lt(y.fst()))),
        case("GT", D::fun2(|x, y| x.fst().gt(y.fst()))),
        case("EQ", D::fun2(|x, y| x.fst().eq(y.fst()))),
    ])?;
    Ok(())
}

pub fn diff(f: impl FnOnce(Arg) -> D) -> Result<D, Error> {
    let expr = D::fun(|x| D(app(d(D::fun(f).0), [x.pair(1.).0]))).0;
    let mut eg = EGraph::default();
    add_to_egraph(&mut eg)?;
    let (sort, val) = eg.eval_expr(&expr)?;

    let mut report = RunReport::default();
    report.updated = true;
    while report.updated {
        report = eg.step_rules("deriv")?;
    }

    let (dag, term, _) = eg.extract_value(&sort, val)?;
    Ok(D(dag.term_to_expr(&term, span!())))
}

pub fn grad(f: impl FnOnce(Arg) -> D) -> Result<D, Error> {
    let expr = D::fun(|v| {
        D::build(v.length(), |i| {
            D(app(d(D::fun(f).0), [v.vector_zip(v.length().one_hot(i)).0]))
        })
    })
    .0;
    let mut eg = EGraph::default();
    add_to_egraph(&mut eg)?;
    let (sort, val) = eg.eval_expr(&expr)?;

    let mut report = RunReport::default();
    report.updated = true;
    while report.updated {
        report = eg.step_rules("deriv")?;
    }

    let (dag, term, _) = eg.extract_value(&sort, val)?;
    Ok(D(dag.term_to_expr(&term, span!())))
}
