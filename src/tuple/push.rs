use crate::sealed::Sealed;
use crate::tuple::pop::TuplePop;

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
pub trait TuplePush<T>: Sized + Sealed {
    /// Result of pushing element `E` to tuple `Self`
    type Res: TuplePop<Rem = Self, Pop = T>;

    fn push(self, element: T) -> Self::Res;
}

impl<T> TuplePush<T> for () {
    type Res = (T,);

    #[inline]
    fn push(self, element: T) -> Self::Res {
        (element,)
    }
}

macro_rules! tuple_impl {
    ($( $types:ident, )*) => {
        impl<$( $types, )* T> TuplePush<T> for ($( $types, )*) {
            type Res = ($( $types, )* T);

            #[inline]
            #[allow(non_snake_case)]
            fn push(self, element: T) -> Self::Res {
                let ($( $types, )*) = self;
                ($( $types, )* element)
            }
        }
    };
}

for_tuples!(A, B, C, D, E, F, G, H, I, J, K, # tuple_impl);
