#[inline]
pub fn untuple<A, F>(f: F) -> Untuple<F>
where
    F: FnOnce<A>,
{
    Untuple::new(f)
}

#[must_use = "function combinators are lazy and do nothing unless called"]
#[derive(Debug, Clone, Copy)]
pub struct Untuple<F>(F);

impl<F> Untuple<F> {
    #[inline]
    pub fn new<A>(f: F) -> Self
    where
        F: FnOnce<A>,
    {
        Untuple(f)
    }

    #[inline]
    pub fn into_inner(self) -> F {
        let Untuple(f) = self;
        f
    }

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
