use egglog::{
    EGraph, Error, RunReport,
    ast::{Command, Rewrite, RustSpan, Span},
    span, var,
};

use crate::{
    deriv::{self, d},
    dsl::{D, DLike},
    f_smooth,
};

fn rewrite(lhs: impl DLike, rhs: impl DLike, subsume: bool) -> Command {
    let rewrite = Rewrite {
        span: span!(),
        lhs: lhs.val().0,
        rhs: rhs.val().0,
        conditions: vec![],
    };
    Command::Rewrite("optim".into(), rewrite, subsume)
}

pub(crate) fn add_to_egraph(eg: &mut EGraph) -> Result<(), Error> {
    eg.parse_and_run_program(Some("optim.egg".into()), include_str!("optim.egg"))?;
    let x = &D(var!("x"));
    let y = &D(var!("y"));
    let z = &D(var!("z"));
    let f = &D(var!("f"));
    let g = &D(var!("g"));
    eg.run_program(vec![
        rewrite(x + 0., x, false),
        rewrite(0. + x, x, false),
        rewrite(x * 1., x, false),
        rewrite(1. * x, x, false),
        rewrite(x * 0., 0., false),
        rewrite(0. * x, 0., false),
        rewrite(x + -y, x - y, false),
        rewrite(x - x, 0., false),
        rewrite(x * y + x * z, x * (y + z), false),
        rewrite(x.build(y).get(z), y.app([z]), true),
        rewrite(x.build(y).length(), x, false),
        rewrite(x.if_then_else(y, y), y, false),
        rewrite(
            f.app([x.if_then_else(y, z)]),
            x.if_then_else(f.app([y]), f.app([z])),
            false,
        ),
        rewrite(D::lam(2, D::var(1)).ifold(x, y), x, false),
        rewrite(x.pair(y).fst(), x, true),
        rewrite(x.pair(y).snd(), y, true),
        rewrite(
            D::lam(
                2,
                f.app([D::var(1).fst(), D::var(0)])
                    .pair(g.app([D::var(1).snd(), D::var(0)])),
            )
            .ifold(x.pair(y), z),
            f.ifold(x, z).pair(g.ifold(y, z)),
            false,
        ),
    ])?;
    Ok(())
}

pub fn optim(x: D) -> Result<D, Error> {
    let expr = x.0;
    let mut eg = EGraph::default();
    f_smooth::add_to_egraph(&mut eg)?;
    add_to_egraph(&mut eg)?;
    let (sort, val) = eg.eval_expr(&expr)?;

    let mut report = RunReport::default();
    report.updated = true;
    while report.updated {
        report = eg.step_rules("optim")?;
    }

    let (dag, term, _) = eg.extract_value(&sort, val)?;
    Ok(D(dag.term_to_expr(&term, span!())))
}

pub fn grad_opt(f: impl DLike) -> Result<D, Error> {
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
    deriv::add_to_egraph(&mut eg)?;
    add_to_egraph(&mut eg)?;
    let (sort, val) = eg.eval_expr(&expr)?;

    let mut report = RunReport::default();
    report.updated = true;
    while report.updated {
        report = eg.step_rules("both")?;
    }

    let (dag, term, _) = eg.extract_value(&sort, val)?;
    Ok(D(dag.term_to_expr(&term, span!())))
}
