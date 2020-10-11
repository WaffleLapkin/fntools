/// Turns functions of n arguments into function of 1 argument —
/// tuple of argument of the original function.
///
/// ## Examples
///
/// ```
/// use fntools::unstable::untuple;
/// use std::ops::Add;
///
/// let args = (1, 2);
/// let fun = untuple(Add::add);
/// assert_eq!(fun(args), 3);
/// ```
#[inline]
pub fn untuple<A, F>(f: F) -> Untuple<F>
where
    F: FnOnce<A>,
{
    Untuple::new(f)
}

/// Turns functions of n arguments into function of 1 argument —
/// tuple of argument of the original function.
#[must_use = "function combinators are lazy and do nothing unless called"]
#[derive(Debug, Clone, Copy)]
pub struct Untuple<F>(F);

impl<F> Untuple<F> {
    /// Turns functions of n arguments into function of 1 argument —
    /// tuple of argument of the original function.
    ///
    /// It's preferred to use [`untuple`] instead.
    #[inline]
    pub fn new<A>(f: F) -> Self
    where
        F: FnOnce<A>,
    {
        Untuple(f)
    }

    /// Returns inner function.
    #[inline]
    pub fn into_inner(self) -> F {
        let Untuple(f) = self;
        f
    }

    /// Returns reference to inner function.
    #[inline]
    pub fn as_inner(&self) -> &F {
        let Untuple(f) = self;
        f
    }
}

impl<A, F> FnOnce<(A,)> for Untuple<F>
where
    F: FnOnce<A>,
{
    type Output = F::Output;

    #[inline]
    extern "rust-call" fn call_once(self, (args,): (A,)) -> Self::Output {
        let Untuple(f) = self;
        let res: F::Output = f.call_once(args);
        res
    }
}

impl<A, F> FnMut<(A,)> for Untuple<F>
where
    F: FnMut<A>,
{
    #[inline]
    extern "rust-call" fn call_mut(&mut self, (args,): (A,)) -> Self::Output {
        let Untuple(f) = self;
        let res: F::Output = f.call_mut(args);
        res
    }
}

impl<A, F> Fn<(A,)> for Untuple<F>
where
    F: Fn<A>,
{
    #[inline]
    extern "rust-call" fn call(&self, (args,): (A,)) -> Self::Output {
        let Untuple(f) = self;
        let res: F::Output = f.call(args);
        res
    }
}
