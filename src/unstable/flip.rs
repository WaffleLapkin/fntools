use crate::tuple::flip::FlipTuple;

/// Flip function arguments
///
/// ```
/// use fntools::unstable::flip::flip;
///
/// let fun = flip(<[_]>::split_at);
/// assert_eq!(fun(2, &[0, 1, 2, 3, 4]), (&[0, 1][..], &[2, 3, 4][..]))
/// ```
pub fn flip<A, F>(f: F) -> Flip<F>
where
    F: FnOnce<A>,
    A: FlipTuple,
{
    Flip::new(f)
}

#[must_use = "function combinators are lazy and do nothing unless called"]
#[derive(Debug, Clone, Copy)]
pub struct Flip<F>(F);

impl<F> Flip<F> {
    /// ```
    /// use fntools::unstable::flip::{flip, Flip};
    ///
    /// let fun = Flip::new(<[_]>::split_at);
    /// assert_eq!(fun(2, &[0, 1, 2, 3, 4]), (&[0, 1][..], &[2, 3, 4][..]))
    /// ```
    pub fn new<A>(f: F) -> Self
    where
        F: FnOnce<A>,
        A: FlipTuple,
    {
        Flip(f)
    }

    pub fn into_inner(self) -> F {
        let Flip(f) = self;
        f
    }

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
    extern "rust-call" fn call(&self, args: A) -> Self::Output {
        let Flip(f) = self;
        let res: F::Output = f.call(args.flip());
        res
    }
}
