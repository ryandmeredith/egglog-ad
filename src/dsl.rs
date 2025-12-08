use egglog::ast::Expr;
use std::ops::{Add, BitAnd, BitOr, Div, Mul, Not, Sub};

use crate::f_smooth::{app_prim, lam, shift, var};

#[derive(Debug, Clone)]
pub struct D(Expr);

macro_rules! fn_impl {
    ($prim:ident, $fun:ident $(, $vis:vis)?) => {
        $($vis)? fn $fun(self) -> Self {
            Self(app_prim(stringify!($prim), [self.0]))
        }
    };
}

macro_rules! bin_impl {
    ($prim:ident, $fun:ident $(, $vis:vis)?) => {
        $($vis)? fn $fun(self, other: Self) -> Self {
            Self(app_prim(stringify!($prim), [self.0, other.0]))
        }
    };
}

macro_rules! op_impl {
    ($prim:ident, $trait:ident, $fun:ident) => {
        impl $trait for D {
            type Output = Self;
            bin_impl!($prim, $fun);
        }
    };
    ($trait:ident, $fun:ident) => {
        op_impl!($trait, $trait, $fun);
    };
}

impl D {
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

    pub fn build(self, f: impl FnOnce(Self) -> Self) -> Self {
        let mut body = f(Self(var(-1))).0;
        shift(0, 1, &mut body);
        Self(app_prim("Build", [self.0, lam(1, body)]))
    }

    pub fn ifold(f: impl FnOnce(Self, Self) -> Self, init: Self, n: Self) -> Self {
        let mut body = f(Self(var(-2)), Self(var(-1))).0;
        shift(0, 2, &mut body);
        Self(app_prim("IFold", [lam(2, body), init.0, n.0]))
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
