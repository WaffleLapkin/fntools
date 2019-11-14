/// Popes element from the **end** of the tuple, producing new tuple.
///
/// Return tuple of remaining tuple and poped element.
///
/// ## Examples
/// ```
/// use fntools::tuple::pop::TuplePop;
///
/// assert_eq!((999,).pop(), ((), 999));
/// assert_eq!((47, "str", 14usize).pop(), ((47, "str"), 14usize));
/// ```
///
/// ```compile_fail
/// use fntools::tuple_pop::TuplePop;
///
/// // There is nothing you can pop from empty tuple,
/// // so this code won't be compiled
/// assert_eq!(().pop(), ());
/// ```
pub trait TuplePop {
    /// Remaining part of the tuple, after popping an element
    type Rem;

    /// Poped element
    type Pop;

    fn pop(self) -> (Self::Rem, Self::Pop);
}

impl<T> TuplePop for (T,) {
    type Rem = ();
    type Pop = T;

    fn pop(self) -> (Self::Rem, Self::Pop) {
        ((), self.0)
    }
}

macro_rules! tuple_impl {
    ($( $types:ident [$e:tt], )* @ $last_ty:ident [$last_e:tt],) => {
        impl<T, $( $types, )* $last_ty> TuplePop for ($( $types, )* $last_ty, T) {
            type Rem = ($( $types, )* $last_ty,);
            type Pop = T;

            fn pop(self) -> (Self::Rem, Self::Pop) {
                ((self.0, $( self.$e ),*), self.$last_e)
            }
        }
    };
}

for_tuples_tt_last!(A [1], B [2], C [3], D [4], E [5], F [6], G [7], H [8], I [9], J [10], K [11], # tuple_impl);
