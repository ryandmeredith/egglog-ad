use egglog::{EGraph, SerializeConfig, ast::Expr};
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    ops::{Add, BitAnd, BitOr, Div, Mul, Neg, Not, Sub},
    path::Path,
};

use crate::f_smooth::{add_to_egraph, app, app_prim, inte, lam, real, shift, var};

#[derive(Debug, Clone)]
pub struct D(pub(crate) Expr);

impl Display for D {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub trait DLike {
    fn val(self) -> D;
}

impl DLike for D {
    fn val(self) -> D {
        self
    }
}

impl DLike for &D {
    fn val(self) -> D {
        self.clone()
    }
}
impl DLike for f64 {
    fn val(self) -> D {
        D::real(self)
    }
}

macro_rules! fn_impl {
    ($prim:ident, $fun:ident $(, $vis:vis)?) => {
        $($vis)? fn $fun(&self) -> D {
            D(app_prim(stringify!($prim), [self.clone().0]))
        }
    };
}

macro_rules! bin_impl {
    ($prim:ident, $fun:ident $(, $vis:vis)?) => {
        $($vis)? fn $fun(&self, other: impl DLike) -> D {
            D(app_prim(stringify!($prim), [self.clone().0, other.val().0]))
        }
    };
}

macro_rules! op_impl {
    ($prim:ident, $trait:ident, $fun:ident) => {
        impl<T: DLike> $trait<T> for D {
            type Output = Self;
            fn $fun(self, other: T) -> Self {
                Self(app_prim(stringify!($prim), [self.0, other.val().0]))
            }
        }
        impl<T: DLike> $trait<T> for &D {
            type Output = D;
            fn $fun(self, other: T) -> D {
                D(app_prim(stringify!($prim), [self.clone().0, other.val().0]))
            }
        }
        impl $trait<D> for f64 {
            type Output = D;
            fn $fun(self, other: D) -> D {
                D(app_prim(stringify!($prim), [real(self), other.0]))
            }
        }
        impl $trait<&D> for f64 {
            type Output = D;
            fn $fun(self, other: &D) -> D {
                D(app_prim(stringify!($prim), [real(self), other.clone().0]))
            }
        }
    };
    ($trait:ident, $fun:ident) => {
        op_impl!($trait, $trait, $fun);
    };
}

impl D {
    pub fn var(n: u32) -> Self {
        Self(var(n))
    }

    pub fn lam(nargs: u32, body: impl DLike) -> Self {
        Self(lam(nargs, body.val().0))
    }

    pub fn app(&self, args: impl IntoIterator<Item = impl DLike>) -> Self {
        Self(app(self.clone().0, args.into_iter().map(|x| x.val().0)))
    }

    pub fn int(i: i64) -> Self {
        Self(inte(i))
    }

    pub fn real(x: f64) -> Self {
        Self(real(x))
    }

    pub fn lift(&self, n: u32) -> Self {
        let mut copy = self.clone();
        shift(0, n.into(), &mut copy.0);
        copy
    }

    pub fn if_then_else(&self, t: impl DLike, f: impl DLike) -> Self {
        Self(app_prim("If", [self.clone().0, t.val().0, f.val().0]))
    }

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

    bin_impl!(Build, build, pub);
    pub fn ifold(&self, init: impl DLike, n: impl DLike) -> Self {
        Self(app_prim("IFold", [self.clone().0, init.val().0, n.val().0]))
    }
    bin_impl!(Get, get, pub);
    fn_impl!(Length, length, pub);

    bin_impl!(Pair, pair, pub);
    fn_impl!(Fst, fst, pub);
    fn_impl!(Snd, snd, pub);

    pub fn vector_zip(&self, other: impl DLike) -> Self {
        let i = &Self::var(0);
        self.length().build(Self::lam(
            1,
            self.lift(1).get(i).pair(other.val().lift(1).get(i)),
        ))
    }

    pub fn one_hot(&self, i: impl DLike) -> Self {
        self.build(Self::lam(
            1,
            Self::var(0).eq(i.val().lift(1)).if_then_else(1., 0.),
        ))
    }

    pub fn sum(&self) -> Self {
        Self::lam(2, Self::var(1) + self.lift(2).get(Self::var(0))).ifold(0., self.length())
    }

    pub fn prod(&self) -> Self {
        Self::lam(2, Self::var(1) * self.lift(2).get(Self::var(0))).ifold(1., self.length())
    }
}

impl Not for D {
    type Output = Self;
    fn not(self) -> Self {
        Self(app_prim("Not", [self.0]))
    }
}

impl Not for &D {
    type Output = D;
    fn not(self) -> D {
        D(app_prim("Not", [self.val().0]))
    }
}

impl Neg for D {
    type Output = Self;
    fn neg(self) -> Self {
        0. - self
    }
}

impl Neg for &D {
    type Output = D;
    fn neg(self) -> D {
        0. - self
    }
}

op_impl!(Add, add);
op_impl!(Sub, sub);
op_impl!(Mul, mul);
op_impl!(Div, div);

op_impl!(And, BitAnd, bitand);
op_impl!(Or, BitOr, bitor);

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
        let mut e = eg.serialize(SerializeConfig::default()).egraph;
        e.inline_leaves();
        e.to_dot_file(path)?;
        Ok(())
    }

    pub fn to_svg_file(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        let mut eg = EGraph::default();
        add_to_egraph(&mut eg)?;
        eg.eval_expr(&self.0)?;
        let mut e = eg.serialize(SerializeConfig::default()).egraph;
        e.inline_leaves();
        e.to_dot_file(path)?;
        Ok(())
    }
}
