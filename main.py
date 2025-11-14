"""Automatic differentiation for egglog."""

from __future__ import annotations

from typing import TYPE_CHECKING

from egglog import (
    Bool,
    BoolLike,
    Expr,
    String,
    StringLike,
    converter,
    f64,
    f64Like,
    function,
    i64,
    i64Like,
)

if TYPE_CHECKING:
    from collections.abc import Callable


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


class Tensor(Expr):  # noqa: PLW1641
    """Tensor value."""

    @classmethod
    def var(cls, name: StringLike) -> Tensor:  # ty: ignore[invalid-return-type]
        """Tensor variable."""

    @classmethod
    def const(cls, val: f64Like) -> Tensor:  # ty: ignore[invalid-return-type]
        """Constant scalar."""

    def __add__(self, other: TensorLike) -> Tensor:  # ty: ignore[invalid-return-type]
        """Add two scalars."""

    def __sub__(self, other: TensorLike) -> Tensor:  # ty: ignore[invalid-return-type]
        """Subtract two scalars."""

    def __mul__(self, other: TensorLike) -> Tensor:  # ty: ignore[invalid-return-type]
        """Multiply two scalars."""

    def __truediv__(self, other: TensorLike) -> Tensor:  # ty: ignore[invalid-return-type]
        """Divide two scalars."""

    def __pow__(self, other: TensorLike) -> Tensor:  # ty: ignore[invalid-return-type]
        """Scalar to a power."""

    def exp(self) -> Tensor:  # ty: ignore[invalid-return-type]
        """Exponentiate a scalar."""

    def log(self) -> Tensor:  # ty: ignore[invalid-return-type]
        """Logarithm of a scalar."""

    def sin(self) -> Tensor:  # ty: ignore[invalid-return-type]
        """Sine of a scalar."""

    def cos(self) -> Tensor:  # ty: ignore[invalid-return-type]
        """Cosine of a scalar."""

    def __lt__(self, other: TensorLike) -> Boolean:  # ty: ignore[invalid-return-type]
        """Less than comparison on scalars."""

    def __eq__(self, other: TensorLike) -> Boolean:  # ty: ignore[invalid-return-type]
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

    def length(self) -> Card:  # ty: ignore[invalid-return-type]
        """Length of arrray."""

    def __getitem__(self, index: IndexLike) -> Tensor:  # ty: ignore[invalid-return-type]
        """Index an array."""


TensorLike = Tensor | StringLike | f64Like | int
converter(String, Tensor, Tensor.var)
converter(f64, Tensor, Tensor.const)
converter(int, Tensor, lambda x: Tensor.const(float(x)))


@function
def build(size: CardLike, fun: Callable[[TensorLike], Tensor]) -> Tensor:  # ty: ignore[invalid-return-type]
    """Build an array."""


@function
def ifold(
    fun: Callable[[TensorLike, IndexLike], Tensor],
    init: TensorLike,
    length: CardLike,
) -> Tensor:  # ty: ignore[invalid-return-type]
    """For loop."""
