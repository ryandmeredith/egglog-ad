use egglog::{
    EGraph, Error, RunReport,
    ast::{RustSpan, Span},
    span,
};

use crate::{dsl::D, f_smooth};

pub(crate) fn add_to_egraph(eg: &mut EGraph) -> Result<(), Error> {
    eg.parse_and_run_program(Some("optim.egg".into()), include_str!("optim.egg"))?;
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
