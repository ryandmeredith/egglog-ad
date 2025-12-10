use egglog::{
    EGraph, Error, RunReport,
    ast::{Command, Rewrite, RustSpan, Span},
    span, var,
};

use crate::{
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
        rewrite(x.build(y).get(z), y.app([z]), false),
        rewrite(x.build(y).length(), x, false),
        rewrite(x.if_then_else(y, y), y, false),
        rewrite(
            f.app([x.if_then_else(y, z)]),
            x.if_then_else(f.app([y]), f.app([z])),
            false,
        ),
        rewrite(D::lam(2, D::var(1)).ifold(x, y), x, false),
        rewrite(x.pair(y).fst(), x, false),
        rewrite(x.pair(y).snd(), y, false),
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
    let mut updated = true;
    while updated {
        let r = eg.step_rules("optim")?;
        updated = r.updated;
        report.union(r);
        println!("{report}");
    }

    let (dag, term, _) = eg.extract_value(&sort, val)?;
    Ok(D(dag.term_to_expr(&term, span!())))
}
