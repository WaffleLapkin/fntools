use crate::tuple::take::TupleTake;

/// Tuple with at least 2 elements.
///
/// This is a workaround for `Curry` to work
pub trait AtLeast2
where
    // There are at least 2 elements, so we can take 2 elements
    Self: TupleTake,
    <Self as TupleTake>::Rem: TupleTake,
{
}

impl<T> AtLeast2 for T
where
    T: TupleTake,
    <T as TupleTake>::Rem: TupleTake,
{
}
