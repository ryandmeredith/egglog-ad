use egglog::{
    EGraph, Error, RunReport,
    ast::{Command, Expr, Rewrite, RustSpan, Span},
    call, span,
};

use crate::{
    dsl::{D, DLike},
    f_smooth,
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
    eg.parse_and_run_program(Some("deriv.egg".into()), include_str!("deriv.egg"))?;
    let x = D::var(1);
    let y = D::var(0);
    eg.run_program(vec![
        case(
            "Add",
            D::lam(2, (x.fst() + y.fst()).pair(x.snd() + y.snd())),
        ),
        case(
            "Sub",
            D::lam(2, (x.fst() - y.fst()).pair(x.snd() - y.snd())),
        ),
        case(
            "Mul",
            D::lam(
                2,
                (x.fst() * y.fst()).pair(x.snd() * y.fst() + x.fst() * y.snd()),
            ),
        ),
        case(
            "Div",
            D::lam(
                2,
                (x.fst() / y.fst()).pair((x.snd() * y.fst() - x.fst() * y.snd()) / y.pow(2.)),
            ),
        ),
        case(
            "Pow",
            D::lam(
                2,
                x.fst().pow(y.fst()).pair(
                    (y.fst() * x.snd() / x.fst() + x.fst().log() * y.snd()) * x.fst().pow(y.fst()),
                ),
            ),
        ),
        case(
            "Exp",
            D::lam(1, x.fst().exp().pair(x.snd() * x.fst().exp())),
        ),
        case("Log", D::lam(1, x.fst().log().pair(x.snd() / x.fst()))),
        case(
            "Sin",
            D::lam(1, x.fst().sin().pair(x.snd() * x.fst().cos())),
        ),
        case(
            "Cos",
            D::lam(1, x.fst().cos().pair(-x.snd() * x.fst().sin())),
        ),
        case("LT", D::lam(2, x.fst().lt(y.fst()))),
        case("GT", D::lam(2, x.fst().gt(y.fst()))),
        case("EQ", D::lam(2, x.fst().eq(y.fst()))),
    ])?;
    Ok(())
}

pub fn diff(f: impl DLike) -> Result<D, Error> {
    let df = D(d(f.val().lift(1).0));
    let expr = D::lam(1, df.app([D::var(0).pair(1.)])).0;
    let mut eg = EGraph::default();
    f_smooth::add_to_egraph(&mut eg)?;
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

pub fn grad(f: impl DLike) -> Result<D, Error> {
    let df = D(d(f.val().lift(2).0));
    let expr = D::lam(
        1,
        D::var(0).length().build(D::lam(
            1,
            df.app([D::var(1).vector_zip(D::var(1).length().one_hot(D::var(0)))]),
        )),
    )
    .0;
    let mut eg = EGraph::default();
    f_smooth::add_to_egraph(&mut eg)?;
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
