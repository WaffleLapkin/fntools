/// Chain two functions.
///
/// Takes functions `f` and `g` and returns `g ∘ f` (in other words something
/// _like_ `|a: A| g(f(a))`.
///
/// # Examples:
/// ```
/// use fntools::unstable::chain::chain;
///
/// let add_two = |a: i32| a + 2;
/// let add_three = |a: i32| a + 3;
/// let add_five = chain(add_two, add_three);
///
/// assert_eq!(add_five(4), 9);
/// ```
///
/// See also:
/// - stable version of this function: [`fntools::chain`]
/// - extension on all functions: [`FnExt::chain`]
///
/// [`FnExt::chain`]: crate::unstable::ext::FnExt::chain
/// [`fntools::chain`]: crate::chain
pub fn chain<A, F, G>(f: F, g: G) -> Chain<F, G>
where
    F: FnOnce<A>,
    G: FnOnce<(F::Output,)>,
{
    Chain::new(f, g)
}

/// Represent composition of 2 functions `G ∘ F`
///
/// Note: `Chain` and [`Compose`] have no differences but argument order.
///
/// For documentation see [`chain`]
///
/// [`Compose`]: crate::unstable::compose::compose
/// [`chain`]: self::chain
#[must_use = "function combinators are lazy and do nothing unless called"]
#[derive(Debug, Clone, Copy)]
pub struct Chain<F, G>(F, G);

impl<F, G> Chain<F, G> {
    pub fn new<A>(f: F, g: G) -> Self
    where
        F: FnOnce<A>,
        G: FnOnce<(F::Output,)>,
    {
        Chain(f, g)
    }

    pub fn into_inner(self) -> (F, G) {
        let Chain(f, g) = self;
        (f, g)
    }

    pub fn as_inner(&self) -> (&F, &G) {
        (&self.0, &self.1)
    }
}

impl<A, F, G> FnOnce<A> for Chain<F, G>
where
    F: FnOnce<A>,
    G: FnOnce<(F::Output,)>,
{
    type Output = G::Output;

    extern "rust-call" fn call_once(self, args: A) -> Self::Output {
        let Chain(f, g) = self;
        let b: F::Output = f.call_once(args);
        let c: G::Output = g(b);
        c
    }
}

impl<A, F, G> FnMut<A> for Chain<F, G>
where
    F: FnMut<A>,
    G: FnMut<(F::Output,)>,
{
    extern "rust-call" fn call_mut(&mut self, args: A) -> Self::Output {
        let Chain(f, g) = self;
        let b: F::Output = f.call_mut(args);
        let c: G::Output = g(b);
        c
    }
}

impl<A, F, G> Fn<A> for Chain<F, G>
where
    F: Fn<A>,
    G: Fn<(F::Output,)>,
{
    extern "rust-call" fn call(&self, args: A) -> Self::Output {
        let Chain(f, g) = self;
        let b: F::Output = f.call(args);
        let c: G::Output = g(b);
        c
    }
}
