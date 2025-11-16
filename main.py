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

simplify = ruleset(name="simplify")


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


class Scalar(Expr, ruleset=simplify):  # noqa: PLW1641
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


class Vector(Expr):
    """Vector expression."""

    @classmethod
    def var(cls, name: StringLike) -> Vector:  # ty:ignore[invalid-return-type]
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


VectorLike = Vector | StringLike
converter(String, Vector, Vector.var)


class Matrix(Expr):
    """Matrix expression."""

    @classmethod
    def var(cls, name: StringLike) -> Matrix:  # ty:ignore[invalid-return-type]
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


MatrixLike = Matrix | StringLike
converter(String, Matrix, Matrix.var)


@function(unextractable=True)
def diff(var: StringLike, fun: ScalarLike) -> Scalar:  # ty:ignore[invalid-return-type]
    """Forward derivative of a scalar-scalar function."""


@ruleset
def deriv(
    x: Scalar,
    y: Scalar,
    v: String,
    w: String,
    c: f64,
) -> Iterable[RewriteOrRule]:
    """Rules for derivatives."""
    yield rewrite(diff(v, Scalar.var(v))).to(Scalar.const(1.0))
    yield rewrite(diff(v, Scalar.var(w))).to(Scalar.const(0.0), v != w)
    yield rewrite(diff(v, Scalar.const(c))).to(Scalar.const(0.0))

    yield rewrite(diff(v, x + y)).to(diff(v, x) + diff(v, y))
    yield rewrite(diff(v, x - y)).to(diff(v, x) - diff(v, y))
    yield rewrite(diff(v, x * y)).to(diff(v, x) * y + x * diff(v, y))
    yield rewrite(diff(v, x / y)).to((diff(v, x) * y - x * diff(v, y)) / y**2)
    yield rewrite(diff(v, x**y)).to((y * diff(v, x) / x + x.log() * diff(v, y)) * x**y)

    yield rewrite(diff(v, x.exp())).to(diff(v, x) * x.exp())
    yield rewrite(diff(v, x.log())).to(diff(v, x) / x)
    yield rewrite(diff(v, x.sin())).to(diff(v, x) * x.cos())
    yield rewrite(diff(v, x.cos())).to(-diff(v, x) * x.sin())
