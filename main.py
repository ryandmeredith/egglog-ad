"""Automatic differentiation for egglog."""

from __future__ import annotations

from collections.abc import Iterable  # noqa: TC003

from egglog import (
    Bool,
    BoolLike,
    Expr,
    RewriteOrRule,
    String,
    StringLike,
    converter,
    f64,
    f64Like,
    function,
    i64,
    i64Like,
    method,
    rewrite,
    ruleset,
)

expand = ruleset()


class Card(Expr):
    """Cardinality (length) value."""

    @classmethod
    def var(cls, var: StringLike) -> Card:  # ty: ignore[invalid-return-type]
        """Cardinality variable."""

    @classmethod
    def const(cls, val: i64Like) -> Card:  # ty: ignore[invalid-return-type]
        """Cardinality constant."""


CardLike = Card | StringLike | i64Like
converter(String, Card, Card.var)
converter(i64, Card, Card.const)


class Index(Expr):
    """Index value."""

    @classmethod
    def var(cls, var: StringLike) -> Index:  # ty: ignore[invalid-return-type]
        """Index variable."""

    @classmethod
    def const(cls, val: i64Like) -> Index:  # ty: ignore[invalid-return-type]
        """Index constant."""


IndexLike = Index | StringLike | i64Like
converter(String, Index, Index.var)
converter(i64, Index, Index.const)


class Boolean(Expr):
    """Boolean value."""

    @classmethod
    def var(cls, var: StringLike) -> Boolean:  # ty: ignore[invalid-return-type]
        """Boolean variable."""

    @classmethod
    def const(cls, val: BoolLike) -> Boolean:  # ty: ignore[invalid-return-type]
        """Boolean constant."""

    def __and__(self, other: BooleanLike) -> Boolean:  # ty: ignore[invalid-return-type]
        """And operator."""

    def __or__(self, other: BooleanLike) -> Boolean:  # ty: ignore[invalid-return-type]
        """Or operator."""

    @property
    def n(self) -> Boolean:  # ty: ignore[invalid-return-type]
        """Not operator."""


BooleanLike = Boolean | StringLike | BoolLike
converter(String, Boolean, Boolean.var)
converter(Bool, Boolean, Boolean.var)


class Scalar(Expr, ruleset=expand):  # noqa: PLW1641
    """Scalar expression."""

    @classmethod
    def var(cls, name: StringLike) -> Scalar:  # ty: ignore[invalid-return-type]
        """Scalar variable."""

    @classmethod
    def const(cls, val: f64Like) -> Scalar:  # ty: ignore[invalid-return-type]
        """Constant scalar."""

    def __add__(self, other: ScalarLike) -> Scalar:  # ty: ignore[invalid-return-type]
        """Add two scalars."""

    def __sub__(self, other: ScalarLike) -> Scalar:  # ty: ignore[invalid-return-type]
        """Subtract two scalars."""

    def __mul__(self, other: ScalarLike) -> Scalar:  # ty: ignore[invalid-return-type]
        """Multiply two scalars."""

    def __truediv__(self, other: ScalarLike) -> Scalar:  # ty: ignore[invalid-return-type]
        """Divide two scalars."""

    def __pow__(self, other: ScalarLike) -> Scalar:  # ty: ignore[invalid-return-type]
        """Scalar to a power."""

    def __neg__(self) -> Scalar:
        """Negatae a scalar."""
        return self * Scalar.const(-1.0)

    def exp(self) -> Scalar:  # ty: ignore[invalid-return-type]
        """Exponentiate a scalar."""

    def log(self) -> Scalar:  # ty: ignore[invalid-return-type]
        """Logarithm of a scalar."""

    def sin(self) -> Scalar:  # ty: ignore[invalid-return-type]
        """Sine of a scalar."""

    def cos(self) -> Scalar:  # ty: ignore[invalid-return-type]
        """Cosine of a scalar."""

    @method(unextractable=True)
    def tan(self) -> Scalar:
        """Tangent of a scalar."""
        return self.sin() / self.cos()

    def __eq__(self, other: ScalarLike) -> Boolean:  # ty: ignore[invalid-return-type]
        """Equality comparison on scalars."""

    @method(unextractable=True)
    def __ne__(self, other: ScalarLike) -> Boolean:
        """Equality comparison on scalars."""
        return (self == other).n

    def __lt__(self, other: ScalarLike) -> Boolean:  # ty: ignore[invalid-return-type]
        """Less than comparison on scalars."""

    @method(unextractable=True)
    def __le__(self, other: ScalarLike) -> Boolean:
        """Less than comparison on scalars."""
        return (self < other) | (self == other)

    @method(unextractable=True)
    def __gt__(self, other: ScalarLike) -> Boolean:
        """Greater than operator."""
        return (self <= other).n

    @method(unextractable=True)
    def __ge__(self, other: ScalarLike) -> Boolean:
        """Greater than operator."""
        return (self < other).n


ScalarLike = Scalar | StringLike | f64Like | int
converter(String, Scalar, Scalar.var)
converter(f64, Scalar, Scalar.const)
converter(int, Scalar, lambda x: Scalar.const(float(x)))


@function
def cond(condition: BooleanLike, if_true: ScalarLike, if_false: ScalarLike) -> Scalar:  # ty: ignore[invalid-return-type]
    """If expression."""


class Vector(Expr, ruleset=expand):
    """Vector expression."""

    @classmethod
    def var(cls, name: StringLike) -> Vector:  # ty: ignore[invalid-return-type]
        """Vector variable."""

    @classmethod
    def build(cls, size: CardLike, index: StringLike, fun: ScalarLike) -> Vector:  # ty: ignore[invalid-return-type]
        """Build a vector."""

    @classmethod
    def ifold(
        cls,
        acc: StringLike,
        index: StringLike,
        fun: ScalarLike,
        init: ScalarLike,
        length: CardLike,
    ) -> Scalar:  # ty: ignore[invalid-return-type]
        """Fold over a vector."""

    def length(self) -> Card:  # ty: ignore[invalid-return-type]
        """Length of a vector."""

    def __getitem__(self, index: IndexLike) -> Scalar:  # ty: ignore[invalid-return-type]
        """Index a vector."""

    @method(unextractable=True)
    @classmethod
    def zero(cls, length: CardLike) -> Vector:
        """Zero vector."""
        return cls.build(length, "x", 0)


VectorLike = Vector | StringLike
converter(String, Vector, Vector.var)


class Matrix(Expr, ruleset=expand):
    """Matrix expression."""

    @classmethod
    def var(cls, name: StringLike) -> Matrix:  # ty: ignore[invalid-return-type]
        """Matrix variable."""

    @classmethod
    def build(cls, size: CardLike, index: StringLike, fun: VectorLike) -> Matrix:  # ty: ignore[invalid-return-type]
        """Build a matrix."""

    @classmethod
    def ifold(
        cls,
        acc: StringLike,
        index: StringLike,
        fun: VectorLike,
        init: VectorLike,
        length: CardLike,
    ) -> Vector:  # ty: ignore[invalid-return-type]
        """Fold over a matrix."""

    def length(self) -> Card:  # ty: ignore[invalid-return-type]
        """Length of a matrix."""

    def __getitem__(self, index: IndexLike) -> Vector:  # ty: ignore[invalid-return-type]
        """Index a matrix."""

    @method(unextractable=True)
    @classmethod
    def zero(cls, m: CardLike, n: CardLike) -> Matrix:
        """Zero matrix."""
        return cls.build(m, "x", Vector.zero(n))


MatrixLike = Matrix | StringLike
converter(String, Matrix, Matrix.var)


@function(unextractable=True)
def diff(var: StringLike, fun: ScalarLike) -> Scalar:  # ty: ignore[invalid-return-type]
    """Forward derivative of a scalar-scalar function."""


@function(unextractable=True)
def vdiff(var: StringLike, fun: VectorLike) -> Vector:  # ty: ignore[invalid-return-type]
    """Forward derivative of a scalar-vector function."""


@function(unextractable=True)
def mdiff(var: StringLike, fun: MatrixLike) -> Matrix:  # ty: ignore[invalid-return-type]
    """Forward derivative of a scalar-matrix function."""


@ruleset
def deriv(  # noqa: PLR0913
    x: Scalar,
    y: Scalar,
    s: String,
    t: String,
    u: String,
    c: f64,
    b: Boolean,
    v: Vector,
    w: Vector,
    m: Matrix,
    i: Index,
    n: Card,
) -> Iterable[RewriteOrRule]:
    """Rules for derivatives."""
    yield rewrite(diff(s, Scalar.var(s))).to(Scalar.const(1.0))
    yield rewrite(diff(s, Scalar.var(t))).to(Scalar.const(0.0), s != t)
    yield rewrite(diff(s, Scalar.const(c))).to(Scalar.const(0.0))
    yield rewrite(diff(s, cond(b, x, y))).to(cond(b, diff(s, x), diff(s, y)))

    yield rewrite(diff(s, x + y)).to(diff(s, x) + diff(s, y))
    yield rewrite(diff(s, x - y)).to(diff(s, x) - diff(s, y))
    yield rewrite(diff(s, x * y)).to(diff(s, x) * y + x * diff(s, y))
    yield rewrite(diff(s, x / y)).to((diff(s, x) * y - x * diff(s, y)) / y**2)
    yield rewrite(diff(s, x**y)).to((y * diff(s, x) / x + x.log() * diff(s, y)) * x**y)

    yield rewrite(diff(s, x.exp())).to(diff(s, x) * x.exp())
    yield rewrite(diff(s, x.log())).to(diff(s, x) / x)
    yield rewrite(diff(s, x.sin())).to(diff(s, x) * x.cos())
    yield rewrite(diff(s, x.cos())).to(-diff(s, x) * x.sin())

    yield rewrite(diff(s, v[i])).to(vdiff(s, v)[i])
    yield rewrite(diff(s, Vector.ifold(t, u, x, y, n))).to(
        Vector.ifold(t, u, diff(s, x), diff(s, y), n),
    )
    yield rewrite(vdiff(s, Vector.var(t))).to(Vector.zero(Vector.var(t).length()))
    yield rewrite(vdiff(s, Vector.build(n, t, x))).to(Vector.build(n, t, diff(s, x)))

    yield rewrite(vdiff(s, m[i])).to(mdiff(s, m)[i])
    yield rewrite(vdiff(s, Matrix.ifold(t, u, v, w, n))).to(
        Matrix.ifold(t, u, vdiff(s, v), vdiff(s, w), n),
    )
    yield rewrite(mdiff(s, Matrix.var(t))).to(
        Matrix.zero(Matrix.var(t).length(), Matrix.var(t)[0].length()),
    )
    yield rewrite(mdiff(s, Matrix.build(n, t, v))).to(Matrix.build(n, t, vdiff(s, v)))
