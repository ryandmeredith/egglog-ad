use egglog::{EGraph, SerializeConfig, ast::Expr};
use std::{
    error::Error,
    ops::{Add, BitAnd, BitOr, Div, Mul, Not, Sub},
    path::Path,
    sync::atomic::{AtomicU32, Ordering},
};

use crate::f_smooth::{add_to_egraph, app_prim, lam, real, var};

#[derive(Debug, Clone)]
pub struct D(Expr);

static DEPTH: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, Clone, Copy)]
pub struct Arg(u32);

pub trait DLike {
    fn val(self) -> D;
}

impl DLike for D {
    fn val(self) -> D {
        self
    }
}

impl DLike for Arg {
    fn val(self) -> D {
        D(var(DEPTH.load(Ordering::Acquire) - self.0))
    }
}

macro_rules! fn_impl {
    ($prim:ident, $fun:ident $(, $vis:vis)?) => {
        $($vis)? fn $fun(self) -> D {
            D(app_prim(stringify!($prim), [self.val().0]))
        }
    };
}

macro_rules! bin_impl {
    ($prim:ident, $fun:ident $(, $vis:vis)?) => {
        $($vis)? fn $fun(self, other: impl DLike) -> D {
            D(app_prim(stringify!($prim), [self.val().0, other.val().0]))
        }
    };
}

macro_rules! op_impl {
    ($prim:ident, $trait:ident, $fun:ident) => {
        impl $trait for D {
            type Output = Self;
            fn $fun(self, other: Self) -> Self {
                Self(app_prim(stringify!($prim), [self.0, other.0]))
            }
        }
        impl $trait<Arg> for D {
            type Output = Self;
            fn $fun(self, other: Arg) -> Self {
                Self(app_prim(stringify!($prim), [self.0, other.val().0]))
            }
        }
        impl $trait for Arg {
            type Output = D;
            fn $fun(self, other: Self) -> D {
                D(app_prim(stringify!($prim), [self.val().0, other.val().0]))
            }
        }
        impl $trait<D> for Arg {
            type Output = D;
            fn $fun(self, other: D) -> D {
                D(app_prim(stringify!($prim), [self.val().0, other.0]))
            }
        }
    };
    ($trait:ident, $fun:ident) => {
        op_impl!($trait, $trait, $fun);
    };
}

impl D {
    pub fn constant(x: f64) -> Self {
        Self(real(x))
    }

    pub fn fun(f: impl FnOnce(Arg) -> Self) -> Self {
        let level = DEPTH.fetch_add(1, Ordering::AcqRel);
        let body = f(Arg(level + 1)).0;
        DEPTH.fetch_sub(1, Ordering::AcqRel);
        Self(lam(1, body))
    }

    bin_impl!(Pow, pow, pub);

    fn_impl!(Exp, exp, pub);
    fn_impl!(Log, log, pub);
    fn_impl!(Sin, sin, pub);
    fn_impl!(Cos, cos, pub);

    bin_impl!(LT, lt, pub);
    bin_impl!(GT, gt, pub);
    bin_impl!(EQ, eq, pub);

    pub fn le(self, other: Self) -> Self {
        !self.gt(other)
    }

    pub fn ge(self, other: Self) -> Self {
        !self.lt(other)
    }

    pub fn ne(self, other: Self) -> Self {
        !self.eq(other)
    }

    pub fn build(self, f: impl FnOnce(Arg) -> Self) -> Self {
        let level = DEPTH.fetch_add(1, Ordering::AcqRel);
        let body = f(Arg(level + 1)).0;
        DEPTH.fetch_sub(1, Ordering::AcqRel);
        Self(app_prim("Build", [self.0, lam(1, body)]))
    }

    pub fn ifold(f: impl FnOnce(Arg, Arg) -> Self, init: Self, n: Self) -> Self {
        let level = DEPTH.fetch_add(2, Ordering::AcqRel);
        let body = f(Arg(level + 1), Arg(level + 2)).0;
        DEPTH.fetch_sub(2, Ordering::AcqRel);
        Self(app_prim("IFold", [lam(2, body), init.0, n.0]))
    }

    bin_impl!(Get, get, pub);
    fn_impl!(Length, length, pub);

    bin_impl!(Pair, pair, pub);
    fn_impl!(Fst, fst, pub);
    fn_impl!(Snd, snd, pub);
}

impl Arg {
    bin_impl!(Pow, pow, pub);

    fn_impl!(Exp, exp, pub);
    fn_impl!(Log, log, pub);
    fn_impl!(Sin, sin, pub);
    fn_impl!(Cos, cos, pub);

    bin_impl!(LT, lt, pub);
    bin_impl!(GT, gt, pub);
    bin_impl!(EQ, eq, pub);

    pub fn le(self, other: impl DLike) -> D {
        !self.gt(other)
    }

    pub fn ge(self, other: impl DLike) -> D {
        !self.lt(other)
    }

    pub fn ne(self, other: impl DLike) -> D {
        !self.eq(other)
    }

    bin_impl!(Get, get, pub);
    fn_impl!(Length, length, pub);

    bin_impl!(Pair, pair, pub);
    fn_impl!(Fst, fst, pub);
    fn_impl!(Snd, snd, pub);
}

op_impl!(Add, add);
op_impl!(Sub, sub);
op_impl!(Mul, mul);
op_impl!(Div, div);

op_impl!(And, BitAnd, bitand);
op_impl!(Or, BitOr, bitor);

impl Not for D {
    type Output = Self;
    fn_impl!(Not, not);
}

impl D {
    pub fn to_json_file(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        let mut eg = EGraph::default();
        add_to_egraph(&mut eg)?;
        eg.eval_expr(&self.0)?;
        eg.serialize(SerializeConfig::default())
            .egraph
            .to_json_file(path)?;
        Ok(())
    }

    pub fn to_dot_file(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        let mut eg = EGraph::default();
        add_to_egraph(&mut eg)?;
        eg.eval_expr(&self.0)?;
        eg.serialize(SerializeConfig::default())
            .egraph
            .to_dot_file(path)?;
        Ok(())
    }

    pub fn to_svg_file(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        let mut eg = EGraph::default();
        add_to_egraph(&mut eg)?;
        eg.eval_expr(&self.0)?;
        eg.serialize(SerializeConfig::default())
            .egraph
            .to_svg_file(path)?;
        Ok(())
    }
}
