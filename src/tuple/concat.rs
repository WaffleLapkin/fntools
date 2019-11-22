use crate::tuple::take::TupleTake;

/// Concatenate tuples.
///
/// ## Examples
/// ```
/// use fntools::tuple::concat::TupleConcat;
///
/// assert_eq!((1, false).concat(("hell", 666)), (1, false, "hell", 666));
/// assert_eq!((0,).concat(((),)), (0, ()));
/// ```
/// Any tuple concatenated with () won't change:
/// ```
/// # use fntools::tuple::concat::TupleConcat;
/// assert_eq!((17, 18).concat(()), (17, 18));
/// assert_eq!(().concat(("he", "eh")), ("he", "eh"));
/// ```
/// The trait is implemented for any tuples which total len <= 12:
/// ```
/// # use fntools::tuple::concat::TupleConcat;
/// assert_eq!((0, 1, 2, 3, 4, 5, 6, 7, 8, 9).concat((10, 11)), (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11));
/// assert_eq!((0, 1, 2, 3, 4, 5).concat((6, 7, 8, 9, 10, 11)), (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11));
/// assert_eq!((0, 1).concat((2, 3, 4, 5, 6, 7, 8, 9, 10, 11)), (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11));
/// ```
/// But isn't implemented for bigger tuples:
/// ```compile_fail
/// assert_eq!((0, 1, 2, 3, 4, 5).concat((6, 7, 8, 9, 10, 11, 12)), (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12));
/// ```
pub trait TupleConcat<T>: Sized {
    /// Result of concatenation
    type Res;

    fn concat(self, other: T) -> Self::Res;
}

impl TupleConcat<()> for () {
    type Res = ();

    fn concat(self, _other: ()) -> Self::Res {}
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

    fn concat(self, _other: ()) -> Self::Res {
        self
    }
}

tuple_concat_impl::concat_impls!(A, B, C, D, E, F, G, H, I, J, K; T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10;);
