/// Popes element from the **end** of the tuple, producing new tuple.
///
/// Return tuple of remaining tuple and poped element.
///
/// ## Examples
/// ```
/// use fntools::tuple_pop::TuplePop;
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

impl<T, A> TuplePop for (A, T) {
    type Rem = (A,);
    type Pop = T;

    fn pop(self) -> (Self::Rem, Self::Pop) {
        ((self.0,), self.1)
    }
}

impl<T, A, B> TuplePop for (A, B, T) {
    type Rem = (A, B);
    type Pop = T;

    fn pop(self) -> (Self::Rem, Self::Pop) {
        ((self.0, self.1), self.2)
    }
}

impl<T, A, B, C> TuplePop for (A, B, C, T) {
    type Rem = (A, B, C);
    type Pop = T;

    fn pop(self) -> (Self::Rem, Self::Pop) {
        ((self.0, self.1, self.2), self.3)
    }
}
