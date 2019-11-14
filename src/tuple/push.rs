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
pub trait TuplePush<T> {
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

impl<A, T> TuplePush<T> for (A,) {
    type Res = (A, T);

    fn push(self, element: T) -> Self::Res {
        (self.0, element)
    }
}

impl<A, B, T> TuplePush<T> for (A, B) {
    type Res = (A, B, T);

    fn push(self, element: T) -> Self::Res {
        (self.0, self.1, element)
    }
}

impl<A, B, C, T> TuplePush<T> for (A, B, C) {
    type Res = (A, B, C, T);

    fn push(self, element: T) -> Self::Res {
        (self.0, self.1, self.2, element)
    }
}
