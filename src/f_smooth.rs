use egglog::{
    ArcSort, EGraph, Error, ExecutionState, Primitive, Value,
    ast::{Expr, Literal, RustSpan, Span},
    call,
    constraint::{SimpleTypeConstraint, TypeConstraint},
    prelude::exprs::{float, int},
    sort::{FunctionContainer, VecContainer},
};

pub(crate) fn add_to_egraph(eg: &mut EGraph) -> Result<(), Error> {
    eg.parse_and_run_program(Some("f-smooth.egg".into()), include_str!("f-smooth.egg"))?;
    let vec_sort = eg
        .get_sort_by_name("ExprVec")
        .expect("ExprVec defined in f-smooth.egg")
        .clone();
    let fn_sort = eg
        .get_sort_by_name("MapFn")
        .expect("MapFn defined in f-smooth.egg")
        .clone();
    eg.add_primitive(Map { vec_sort, fn_sort });
    Ok(())
}

pub fn lam(nargs: u32, body: Expr) -> Expr {
    call!("Lam", [int(nargs.into()), body])
}

pub fn app(fun: Expr, args: impl IntoIterator<Item = Expr>) -> Expr {
    call!("App", [fun, call!("vec-of", args)])
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

#[derive(Clone)]
struct Map {
    vec_sort: ArcSort,
    fn_sort: ArcSort,
}

impl Primitive for Map {
    fn name(&self) -> &str {
        "map"
    }

    fn get_type_constraints(&self, span: &Span) -> Box<dyn TypeConstraint> {
        SimpleTypeConstraint::new(
            "map",
            vec![
                self.fn_sort.clone(),
                self.vec_sort.clone(),
                self.vec_sort.clone(),
            ],
            span.clone(),
        )
        .into_box()
    }

    fn apply(&self, exec_state: &mut ExecutionState, args: &[Value]) -> Option<Value> {
        let cv = exec_state.container_values();
        let [f, v] = args else { return None };
        let new = {
            let fun = cv.get_val::<FunctionContainer>(*f)?;
            let vec = cv.get_val::<VecContainer>(*v)?;
            VecContainer {
                do_rebuild: true,
                data: Option::from_iter(vec.data.iter().map(|&x| fun.apply(exec_state, &[x])))?,
            }
        };
        Some(cv.register_val(new, exec_state))
    }
}
