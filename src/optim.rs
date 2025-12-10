use egglog::{
    ArcSort, EGraph, Error, ExecutionState, Primitive, RunReport, Value,
    ast::{Command, Rewrite, RustSpan, Span},
    constraint::{SimpleTypeConstraint, TypeConstraint},
    sort::SetContainer,
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
    eg.declare_sort("VarSet", &Some(("Set".into(), vec![var!("i64")])), span!())?;
    let sort = eg
        .get_sort_by_name("VarSet")
        .expect("VarSet already defined")
        .clone();
    eg.add_primitive(SetShift { sort });
    eg.parse_and_run_program(Some("optim.egg".into()), include_str!("optim.egg"))?;
    let x = &D(var!("x"));
    let y = &D(var!("y"));
    let z = &D(var!("z"));
    let f = &D(var!("f"));
    let g = &D(var!("g"));
    eg.run_program(vec![
        rewrite(x + 0., x, false),
        rewrite(x * 1., x, false),
        rewrite(x * 0., 0., false),
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
        rewrite(x.eq(y), y.eq(x), false),
        rewrite(x + y, y + x, false),
        rewrite(x * y, y * x, false),
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

#[derive(Clone)]
struct SetShift {
    sort: ArcSort,
}

impl Primitive for SetShift {
    fn name(&self) -> &str {
        "set-shift"
    }

    fn get_type_constraints(&self, span: &Span) -> Box<dyn TypeConstraint> {
        SimpleTypeConstraint::new(
            "set-shift",
            vec![self.sort.clone(), self.sort.clone()],
            span.clone(),
        )
        .into_box()
    }

    fn apply(&self, exec_state: &mut ExecutionState, args: &[Value]) -> Option<Value> {
        let [arg] = args else { return None };
        let cv = exec_state.container_values();
        let bv = exec_state.base_values();
        let new = SetContainer {
            do_rebuild: true,
            data: cv
                .get_val::<SetContainer>(*arg)?
                .data
                .iter()
                .filter_map(|x| {
                    let x: i64 = bv.unwrap(*x);
                    if x == 0 { None } else { Some(bv.get(x - 1)) }
                })
                .collect(),
        };
        Some(cv.register_val(new, exec_state))
    }
}
