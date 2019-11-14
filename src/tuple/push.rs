use crate::sealed::Sealed;

/// Push element to the **end** of the tuple, producing new tuple.
///
/// ## Example
/// ```
/// use fntools::tuple::push::TuplePush;
///
/// assert_eq!(().push(1), (1,));
/// assert_eq!((999,).push("str"), (999, "str"));
/// assert_eq!((47, "str", 14usize).push(true), (47, "str", 14usize, true));
/// ```
pub trait TuplePush<T>: Sealed {
    /// Result of pushing element `E` to tuple `Self`
    type Res;

    fn push(self, element: T) -> Self::Res;
}

impl<T> TuplePush<T> for () {
    type Res = (T,);

    fn push(self, element: T) -> Self::Res {
        (element,)
    }
}

macro_rules! tuple_impl {
    ($( $types:ident [$e:tt], )*) => {
        impl<$( $types, )* T> TuplePush<T> for ($( $types, )*) {
            type Res = ($( $types, )* T);

            fn push(self, element: T) -> Self::Res {
                ($( self.$e, )* element)
            }
        }
    };
}

for_tuples_tt!(A [0], B [1], C [2], D [3], E [4], F [5], G [6], H[7], I[8], J[9], K[10], # tuple_impl);
