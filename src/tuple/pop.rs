use crate::{sealed::Sealed, tuple::push::TuplePush};

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
pub trait TuplePop: Sized + Sealed {
    /// Remaining part of the tuple, after popping an element
    type Rem: TuplePush<Self::Pop, Res = Self>;

    /// Poped element
    type Pop;

    /// Pop element from tuple.
    fn pop(self) -> (Self::Rem, Self::Pop);
}

impl<T> TuplePop for (T,) {
    type Pop = T;
    type Rem = ();

    #[inline]
    fn pop(self) -> (Self::Rem, Self::Pop) { ((), self.0) }
}

macro_rules! tuple_impl {
    ($( $types:ident, )*) => {
        impl<T, $( $types, )*> TuplePop for ($( $types, )* T) {
            type Rem = ($( $types, )*);
            type Pop = T;

            #[inline]
            #[allow(non_snake_case)]
            fn pop(self) -> (Self::Rem, Self::Pop) {
                let ($( $types, )* pop) = self;
                (($( $types, )*), pop)
            }
        }
    };
}

for_tuples!(A, B, C, D, E, F, G, H, I, J, K, # tuple_impl);
