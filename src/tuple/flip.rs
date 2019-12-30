#![allow(clippy::unit_cmp)]

use crate::sealed::Sealed;

/// Flips tuple, so first element becomes last, last becomes first, 2-nd becomes
/// 2-nd from the end and so on.
///
/// ## Examples
/// ```
/// use fntools::tuple::flip::FlipTuple;
///
/// assert_eq!((1, "hello").flip(), ("hello", 1));
/// assert_eq!((true, 42, ()).flip(), ((), 42, true));
/// ```
///
/// ```
/// use fntools::tuple::flip::FlipTuple;
///
/// let tuple = (17, (), false, "OwO");
/// assert_eq!(tuple.flip().flip(), tuple);
/// ```
pub trait FlipTuple: Sized + Sealed {
    /// Result of flipping the tuple
    type Res: FlipTuple<Res = Self>;

    fn flip(self) -> Self::Res;
}

#[cfg(test)]
mod tests {
    use crate::tuple::flip::FlipTuple;

    #[test]
    fn test() {
        // I don't know proposes of these tests

        // arity 0
        assert_eq!(().flip(), ());

        // arity 1
        assert_eq!((5,).flip(), (5,));
        assert_eq!((true,).flip(), (true,));
        assert_eq!((false,).flip(), (false,));

        // arity 2
        assert_eq!((16, false).flip(), (false, 16));
        assert_eq!((true, 42).flip(), (42, true));

        // arity 3
        assert_eq!((10, "h", true).flip(), (true, "h", 10));
        assert_eq!((1, 2, 3).flip(), (3, 2, 1));

        // arity 12
        assert_eq!(
            (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12).flip(),
            (12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1)
        );
    }
}

impl FlipTuple for () {
    type Res = ();

    #[inline]
    fn flip(self) -> Self::Res {}
}

impl<A> FlipTuple for (A,) {
    type Res = (A,);

    #[inline]
    fn flip(self) -> Self::Res { (self.0,) }
}

impl<A, B> FlipTuple for (A, B) {
    type Res = (B, A);

    #[inline]
    fn flip(self) -> Self::Res { (self.1, self.0) }
}

impl<A, B, C> FlipTuple for (A, B, C) {
    type Res = (C, B, A);

    #[inline]
    fn flip(self) -> Self::Res { (self.2, self.1, self.0) }
}

impl<A, B, C, D> FlipTuple for (A, B, C, D) {
    type Res = (D, C, B, A);

    #[inline]
    fn flip(self) -> Self::Res { (self.3, self.2, self.1, self.0) }
}

impl<A, B, C, D, E> FlipTuple for (A, B, C, D, E) {
    type Res = (E, D, C, B, A);

    #[inline]
    fn flip(self) -> Self::Res { (self.4, self.3, self.2, self.1, self.0) }
}

impl<A, B, C, D, E, F> FlipTuple for (A, B, C, D, E, F) {
    type Res = (F, E, D, C, B, A);

    #[inline]
    fn flip(self) -> Self::Res { (self.5, self.4, self.3, self.2, self.1, self.0) }
}

impl<A, B, C, D, E, F, G> FlipTuple for (A, B, C, D, E, F, G) {
    type Res = (G, F, E, D, C, B, A);

    #[inline]
    fn flip(self) -> Self::Res { (self.6, self.5, self.4, self.3, self.2, self.1, self.0) }
}

impl<A, B, C, D, E, F, G, H> FlipTuple for (A, B, C, D, E, F, G, H) {
    type Res = (H, G, F, E, D, C, B, A);

    #[inline]
    fn flip(self) -> Self::Res {
        (
            self.7, self.6, self.5, self.4, self.3, self.2, self.1, self.0,
        )
    }
}

impl<A, B, C, D, E, F, G, H, I> FlipTuple for (A, B, C, D, E, F, G, H, I) {
    type Res = (I, H, G, F, E, D, C, B, A);

    #[inline]
    fn flip(self) -> Self::Res {
        (
            self.8, self.7, self.6, self.5, self.4, self.3, self.2, self.1, self.0,
        )
    }
}

impl<A, B, C, D, E, F, G, H, I, J> FlipTuple for (A, B, C, D, E, F, G, H, I, J) {
    type Res = (J, I, H, G, F, E, D, C, B, A);

    #[inline]
    fn flip(self) -> Self::Res {
        (
            self.9, self.8, self.7, self.6, self.5, self.4, self.3, self.2, self.1, self.0,
        )
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K> FlipTuple for (A, B, C, D, E, F, G, H, I, J, K) {
    type Res = (K, J, I, H, G, F, E, D, C, B, A);

    #[inline]
    fn flip(self) -> Self::Res {
        (
            self.10, self.9, self.8, self.7, self.6, self.5, self.4, self.3, self.2, self.1, self.0,
        )
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L> FlipTuple for (A, B, C, D, E, F, G, H, I, J, K, L) {
    type Res = (L, K, J, I, H, G, F, E, D, C, B, A);

    #[inline]
    fn flip(self) -> Self::Res {
        (
            self.11, self.10, self.9, self.8, self.7, self.6, self.5, self.4, self.3, self.2,
            self.1, self.0,
        )
    }
}
