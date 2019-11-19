use crate::sealed::Sealed;
use crate::tuple::take::TupleTake;

/// Append element to the **start** of the tuple, producing new tuple.
///
/// ## Example
/// ```
/// use fntools::tuple::append::TupleAppend;
///
/// assert_eq!(().append(1), (1,));
/// assert_eq!((999,).append("str"), ("str", 999));
/// assert_eq!((47, "str", 14usize).append(true), (true, 47, "str", 14usize));
/// ```
pub trait TupleAppend<E>: Sized + Sealed {
    /// Result of the appending element `E` to tuple `Self`
    type Res: TupleTake<Take = E, Rem = Self>;

    fn append(self, element: E) -> Self::Res;
}

impl<T> TupleAppend<T> for () {
    type Res = (T,);

    #[inline]
    fn append(self, element: T) -> (T,) {
        (element,)
    }
}

macro_rules! tuple_impl {
    ($( $types:ident [$e:tt], )*) => {
        impl<T, $( $types, )*> TupleAppend<T> for ($( $types, )*) {
            type Res = (T, $( $types, )*);

            #[inline]
            fn append(self, element: T) -> Self::Res {
                (element, $( self.$e, )*)
            }
        }
    };
}

for_tuples_tt!(A [0], B [1], C [2], D [3], E [4], F [5], G [6], H[7], I[8], J[9], K[10], # tuple_impl);
