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
pub trait TupleTake {
    /// Remaining part of the tuple, after taking an element
    type Rem;

    /// Taken element
    type Take;

    fn take(self) -> (Self::Take, Self::Rem);
}

impl<T> TupleTake for (T,) {
    type Rem = ();

    type Take = T;

    fn take(self) -> (Self::Take, Self::Rem) {
        (self.0, ())
    }
}

impl<T, A> TupleTake for (T, A) {
    type Rem = (A,);
    type Take = T;

    fn take(self) -> (Self::Take, Self::Rem) {
        (self.0, (self.1,))
    }
}

impl<T, A, B> TupleTake for (T, A, B) {
    type Rem = (A, B);
    type Take = T;

    fn take(self) -> (Self::Take, Self::Rem) {
        (self.0, (self.1, self.2))
    }
}

impl<T, A, B, C> TupleTake for (T, A, B, C) {
    type Rem = (A, B, C);
    type Take = T;

    fn take(self) -> (Self::Take, Self::Rem) {
        (self.0, (self.1, self.2, self.3))
    }
}
