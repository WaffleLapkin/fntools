use crate::tuple::flip::FlipTuple;

/// Flips argument order of `self`.
///
/// # Example
/// ```
/// use fntools::unstable::flip;
///
/// let fun = |a: &str, b: i32, c: char| format!("{}{}{}", a, b, c);
/// let fun = flip(fun);
///
/// assert_eq!(fun('c', 17, "hello, "), "hello, 17c")
/// ```
pub fn flip<A, F>(f: F) -> Flip<F>
where
    F: FnOnce<A>,
    A: FlipTuple,
{
    Flip::new(f)
}

/// Represents function `F` with flipped argument order.
///
/// For documentation see [`flip`].
#[must_use = "function combinators are lazy and do nothing unless called"]
#[derive(Debug, Clone, Copy)]
pub struct Flip<F>(F);

impl<F> Flip<F> {
    /// Creates curried function `f`.
    ///
    /// It's preferred to use [`flip`] instead.
    #[inline]
    pub fn new<A>(f: F) -> Self
    where
        F: FnOnce<A>,
        A: FlipTuple,
    {
        Flip(f)
    }

    /// Returns inner function.
    #[inline]
    pub fn into_inner(self) -> F {
        let Flip(f) = self;
        f
    }

    /// Returns reference to inner function.
    #[inline]
    pub fn as_inner(&self) -> &F {
        let Flip(f) = self;
        f
    }
}

impl<A, F> FnOnce<A> for Flip<F>
where
    F: FnOnce<A::Res>,
    A: FlipTuple,
{
    type Output = F::Output;

    #[inline]
    extern "rust-call" fn call_once(self, args: A) -> Self::Output {
        let Flip(f) = self;
        let res: F::Output = f.call_once(args.flip());
        res
    }
}

impl<A, F> FnMut<A> for Flip<F>
where
    F: FnMut<A::Res>,
    A: FlipTuple,
{
    #[inline]
    extern "rust-call" fn call_mut(&mut self, args: A) -> Self::Output {
        let Flip(f) = self;
        let res: F::Output = f.call_mut(args.flip());
        res
    }
}

impl<A, F> Fn<A> for Flip<F>
where
    F: Fn<A::Res>,
    A: FlipTuple,
{
    #[inline]
    extern "rust-call" fn call(&self, args: A) -> Self::Output {
        let Flip(f) = self;
        let res: F::Output = f.call(args.flip());
        res
    }
}
