/// Compose two functions.
///
/// Takes functions `f` and `g` and returns `f ∘ g` (in other words something
/// _like_ `|a: A| f(g(a))`.
///
/// # Examples:
///
/// ```
/// use fntools::unstable::compose;
///
/// let add_two = |a: i32| a + 2;
/// let add_three = |a: i32| a + 3;
/// let add_five = compose(add_two, add_three);
///
/// assert_eq!(add_five(4), 9);
/// ```
///
/// See also:
/// - stable version of this function: [`fntools::compose`]
/// - not untupling version of this function: [`compose`]
/// - extension on all functions: [`FnExt::compose`]
///
///
/// [`FnExt::compose`]: crate::unstable::ext::FnExt::compose
/// [`compose`]: super::compose::compose
/// [`fntools::compose`]: crate::compose
#[inline]
pub fn compose<A, F, G>(f: F, g: G) -> Compose<F, G>
where
    F: FnOnce<(G::Output,)>,
    G: FnOnce<A>,
{
    Compose::new(f, g)
}

/// Represent composition of 2 functions `F ∘ G`
///
/// For documentation see [`compose`]
///
/// [`compose`]: self::compose
#[must_use = "function combinators are lazy and do nothing unless called"]
#[derive(Debug, Clone, Copy)]
pub struct Compose<F, G> {
    f: F,
    g: G,
}

impl<F, G> Compose<F, G> {
    #[inline]
    pub fn new<A>(f: F, g: G) -> Self
    where
        F: FnOnce<(G::Output,)>,
        G: FnOnce<A>,
    {
        Compose { f, g }
    }

    #[inline]
    pub fn into_inner(self) -> (F, G) {
        let Compose { f, g } = self;
        (f, g)
    }

    #[inline]
    pub fn as_inner(&self) -> (&F, &G) {
        let Compose { f, g } = self;
        (f, g)
    }
}

impl<A, F, G> FnOnce<A> for Compose<F, G>
where
    F: FnOnce<(G::Output,)>,
    G: FnOnce<A>,
{
    type Output = F::Output;

    #[inline]
    extern "rust-call" fn call_once(self, args: A) -> Self::Output {
        let Compose { f, g } = self;
        let b: G::Output = g.call_once(args);
        let c: F::Output = f(b);
        c
    }
}

impl<A, F, G> FnMut<A> for Compose<F, G>
where
    F: FnMut<(G::Output,)>,
    G: FnMut<A>,
{
    #[inline]
    extern "rust-call" fn call_mut(&mut self, args: A) -> Self::Output {
        let Compose { f, g } = self;
        let b: G::Output = g.call_mut(args);
        let c: F::Output = f(b);
        c
    }
}

impl<A, F, G> Fn<A> for Compose<F, G>
where
    F: Fn<(G::Output,)>,
    G: Fn<A>,
{
    #[inline]
    extern "rust-call" fn call(&self, args: A) -> Self::Output {
        let Compose { f, g } = self;
        let b: G::Output = g.call(args);
        let c: F::Output = f(b);
        c
    }
}
