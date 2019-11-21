use crate::tuple::take::TupleTake;

/// Concatenate tuples.
///
/// ## Example
/// ```
///
/// ```
pub trait TupleConcat<T>: Sized {
    /// Result of concatenation
    type Res;

    fn concat(self, other: T) -> Self::Res;
}

impl TupleConcat<()> for () {
    type Res = ();

    fn concat(self, other: ()) -> Self::Res {}
}

impl<T> TupleConcat<T> for ()
where
    T: TupleTake, // Any tuple with at least one element
{
    type Res = T;

    fn concat(self, other: T) -> Self::Res {
        other
    }
}

impl<T> TupleConcat<()> for T
where
    T: TupleTake, // Any tuple with at least one element
{
    type Res = T;

    fn concat(self, other: ()) -> Self::Res {
        self
    }
}
