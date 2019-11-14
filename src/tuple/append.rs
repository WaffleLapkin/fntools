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
pub trait TupleAppend<E> {
    /// Result of the appending element `E` to tuple `Self`
    type Res;

    fn append(self, element: E) -> Self::Res;
}

impl<T> TupleAppend<T> for () {
    type Res = (T,);

    fn append(self, element: T) -> (T,) {
        (element,)
    }
}

impl<T, A> TupleAppend<T> for (A,) {
    type Res = (T, A);

    fn append(self, element: T) -> (T, A) {
        (element, self.0)
    }
}

impl<T, A, B> TupleAppend<T> for (A, B) {
    type Res = (T, A, B);

    fn append(self, element: T) -> (T, A, B) {
        (element, self.0, self.1)
    }
}

impl<T, A, B, C> TupleAppend<T> for (A, B, C) {
    type Res = (T, A, B, C);

    fn append(self, element: T) -> (T, A, B, C) {
        (element, self.0, self.1, self.2)
    }
}
