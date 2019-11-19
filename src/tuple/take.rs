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
    ($( $types:ident [$e:tt], )*) => {
        impl<T, $( $types, )*> TupleTake for (T, $( $types, )*) {
            type Rem = ($( $types, )*);
            type Take = T;

            #[inline]
            fn take(self) -> (Self::Take, Self::Rem) {
                (self.0, ($( self.$e, )*))
            }
        }
    };
}

for_tuples_tt!(A [1], B [2], C [3], D [4], E [5], F [6], G [7], H [8], I [9], J [10], K [11], # tuple_impl);
