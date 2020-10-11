use crate::{sealed::Sealed, tuple::take::TupleTake};

/// Append element to the **start** of the tuple, producing new tuple.
///
/// ## Example
/// ```
/// use fntools::tuple::append::TupleAppend;
///
/// assert_eq!(().append(1), (1,));
/// assert_eq!((999,).append("str"), ("str", 999));
/// assert_eq!(
///     (47, "str", 14usize).append(true),
///     (true, 47, "str", 14usize)
/// );
/// ```
pub trait TupleAppend<E>: Sized + Sealed {
    /// Result of the appending element `E` to tuple `Self`
    type Res: TupleTake<Take = E, Rem = Self>;

    /// Append element to tuple.
    fn append(self, element: E) -> Self::Res;
}

impl<T> TupleAppend<T> for () {
    type Res = (T,);

    #[inline]
    fn append(self, element: T) -> (T,) { (element,) }
}

macro_rules! tuple_impl {
    ($( $types:ident, )*) => {
        impl<T, $( $types, )*> TupleAppend<T> for ($( $types, )*) {
            type Res = (T, $( $types, )*);

            #[inline]
            #[allow(non_snake_case)]
            fn append(self, element: T) -> Self::Res {
                let ($( $types, )*) = self;
                (element, $( $types, )*)
            }
        }
    };
}

for_tuples!(A, B, C, D, E, F, G, H, I, J, K, # tuple_impl);
