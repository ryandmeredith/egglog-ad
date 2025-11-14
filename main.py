"""Automatic differentiation for egglog."""

from __future__ import annotations

from egglog import (
    Bool,
    BoolLike,
    Expr,
    String,
    StringLike,
    converter,
    f64,
    f64Like,
    i64,
    i64Like,
)


class Card(Expr):
    """Cardinality (length) value."""

    @classmethod
    def var(cls, var: StringLike) -> Card:
        """Cardinality variable."""

    @classmethod
    def const(cls, val: i64Like) -> Card:
        """Cardinality constant."""


CardLike = Card | StringLike | i64Like
converter(String, Card, Card.var)
converter(i64, Card, Card.const)


class Index(Expr):
    """Index value."""

    @classmethod
    def var(cls, var: StringLike) -> Index:
        """Index variable."""

    @classmethod
    def const(cls, val: i64Like) -> Index:
        """Index constant."""


IndexLike = Index | StringLike | i64Like
converter(String, Index, Index.var)
converter(i64, Index, Index.const)


class Boolean(Expr):
    """Boolean value."""

    @classmethod
    def var(cls, var: StringLike) -> Boolean:
        """Boolean variable."""

    @classmethod
    def const(cls, val: BoolLike) -> Boolean:
        """Boolean constant."""

    def __and__(self, other: BooleanLike) -> Boolean:
        """And operator."""

    def __or__(self, other: BooleanLike) -> Boolean:
        """Or operator."""

    @property
    def n(self) -> Boolean:
        """Not operator."""


BooleanLike = Boolean | StringLike | BoolLike
converter(String, Boolean, Boolean.var)
converter(Bool, Boolean, Boolean.var)


class Tensor(Expr):
    """Tensor value."""

    @classmethod
    def var(cls, name: StringLike) -> Tensor:
        """Tensor variable."""

    @classmethod
    def const(cls, val: f64Like) -> Tensor:
        """Constant scalar."""

    def __add__(self, other: TensorLike) -> Tensor:
        """Add two scalars."""

    def __sub__(self, other: TensorLike) -> Tensor:
        """Subtract two scalars."""

    def __mul__(self, other: TensorLike) -> Tensor:
        """Multiply two scalars."""

    def __truediv__(self, other: TensorLike) -> Tensor:
        """Divide two scalars."""

    def __pow__(self, other: TensorLike) -> Tensor:
        """Scalar to a power."""

    def exp(self) -> Tensor:
        """Exponentiate a scalar."""

    def log(self) -> Tensor:
        """Logarithm of a scalar."""

    def sin(self) -> Tensor:
        """Sine of a scalar."""

    def cos(self) -> Tensor:
        """Cosine of a scalar."""

    def __lt__(self, other: TensorLike) -> Boolean:
        """Less than comparison on scalars."""

    def __eq__(self, other: TensorLike) -> Boolean:
        """Equality comparison on scalars."""

    def __ne__(self, other: TensorLike) -> Boolean:
        """Not equal operator."""
        return (self == other).n

    def __le__(self, other: TensorLike) -> Boolean:
        """Less than or equal operator."""
        return (self < other) | (self == other)

    def __gt__(self, other: TensorLike) -> Boolean:
        """Greater than operator."""
        return (self <= other).n

    def __ge__(self, other: TensorLike) -> Boolean:
        """Greater than or equal operator."""
        return (self < other).n

    @classmethod
    def build(cls, size: CardLike, index_var: StringLike, fun: TensorLike) -> Tensor:
        """Build an array."""


TensorLike = Tensor | StringLike | f64Like | int
converter(String, Tensor, Tensor.var)
converter(f64, Tensor, Tensor.const)
converter(int, Tensor, lambda x: Tensor.const(float(x)))
