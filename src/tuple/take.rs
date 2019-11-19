use crate::sealed::Sealed;
use crate::tuple::append::TupleAppend;

/// Takes element from the **start** of the tuple, producing new tuple.
///
/// Return tuple of taked element and remaining tuple.
///
/// ## Examples
/// ```
/// use fntools::tuple::take::TupleTake;
///
/// assert_eq!((999,).take(), (999, ()));
/// assert_eq!((47, "str", 14usize).take(), (47, ("str", 14usize)));
/// ```
///
/// ```compile_fail
/// use fntools::tuple_take::TupleTake;
///
/// // There is nothing you can take from empty tuple,
/// // so this code won't be compiled
/// assert_eq!(().take(), ());
/// ```
pub trait TupleTake: Sized + Sealed {
    /// Remaining part of the tuple, after taking an element
    type Rem: TupleAppend<Self::Take, Res = Self>;

    /// Taken element
    type Take;

    fn take(self) -> (Self::Take, Self::Rem);
}

impl<T> TupleTake for (T,) {
    type Rem = ();

    type Take = T;

    #[inline]
    fn take(self) -> (Self::Take, Self::Rem) {
        (self.0, ())
    }
}

macro_rules! tuple_impl {
    ($( $types:ident, )*) => {
        impl<T, $( $types, )*> TupleTake for (T, $( $types, )*) {
            type Rem = ($( $types, )*);
            type Take = T;

            #[inline]
            #[allow(non_snake_case)]
            fn take(self) -> (Self::Take, Self::Rem) {
                let (take, $( $types, )*) = self;
                (take, ($( $types, )*))
            }
        }
    };
}

for_tuples!(A, B, C, D, E, F, G, H, I, J, K, # tuple_impl);
