/// Flips tuple, so first element becomes last, last becomes first, 2-nd becomes
/// 2-nd from the end and so on.
///
/// ## Example
/// ```
/// use fntools::flip_tuple::FlipTuple;
///
/// assert_eq!((1, "hello").flip_tuple(), ("hello", 1));
/// assert_eq!((true, 42, ()).flip_tuple(), ((), 42, true));
/// ```
/// ## Limitations
/// By now there are implementations only for tuples of arity 3 or less
pub trait FlipTuple {
    type Res;

    fn flip_tuple(self) -> Self::Res;
}

impl FlipTuple for () {
    type Res = ();

    fn flip_tuple(self) {}
}

impl<A> FlipTuple for (A,) {
    type Res = (A,);

    fn flip_tuple(self) -> Self::Res {
        self
    }
}

impl<A, B> FlipTuple for (A, B) {
    type Res = (B, A);

    fn flip_tuple(self) -> Self::Res {
        (self.1, self.0)
    }
}

impl<A, B, C> FlipTuple for (A, B, C) {
    type Res = (C, B, A);

    fn flip_tuple(self) -> Self::Res {
        (self.2, self.1, self.0)
    }
}
// TODO: macro gen

#[cfg(test)]
mod tests {
    use crate::flip_tuple::FlipTuple;

    #[test]
    fn test() {
        // I don't know proposes of these tests

        // arity 0
        assert_eq!(().flip_tuple(), ());

        // arity 1
        assert_eq!((5,).flip_tuple(), (5,));
        assert_eq!((true,).flip_tuple(), (true,));
        assert_eq!((false,).flip_tuple(), (false,));

        // arity 2
        assert_eq!((16, false).flip_tuple(), (false, 16));
        assert_eq!((true, 42).flip_tuple(), (42, true));

        // arity 3
        assert_eq!((10, "h", true).flip_tuple(), (true, "h", 10));
        assert_eq!((1, 2, 3).flip_tuple(), (3, 2, 1));
    }
}
