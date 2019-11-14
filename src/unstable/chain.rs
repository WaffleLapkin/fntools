use std::{
    fmt::{Debug, Error, Formatter},
    marker::PhantomData,
};

use crate::tuple::auto_tuple::AutoTuple;

/// Chain two functions.
///
/// Takes functions f and g and returns `g ∘ f` (in other words something
/// like `|a: A| g(f(a))`.
///
/// # Examples:
/// ```
/// use fntools::unstable::chain;
///
/// let add_two = |a: i32| a + 2;
/// let add_three = |a: i32| a + 3;
/// let add_five = chain(add_two, add_three);
///
/// assert_eq!(add_five(4), 9);
/// ```
///
/// <a name="second_example"></a> `chain` also work with multi-argument
/// functions:
/// ```
/// use fntools::unstable::chain;
///
/// // very bad impl of `checked_add`
/// let my_checked_add = chain(i32::overflowing_add, |res, over| if over { None } else { Some(res) });
/// //    return `(i32, bool)` ---- ^^^^^^^^^^^^^^^   ^^^^^^^^^---- note: no destructing needed
/// assert_eq!(my_checked_add(8, 16), Some(24));
/// assert_eq!(my_checked_add(std::i32::MAX, 1), None);
/// ```
///
/// Note the order:
/// ```
/// use fntools::unstable::chain;
///
/// let to_16 = |i: i8| i16::from(i);
/// let to_32 = |i: i16| i32::from(i);
/// let to_64 = |i: i32| i64::from(i);
///
/// // execution order: to_16 -> to_32 -> to_64
/// let i8_to_i64 = chain(to_16, chain(to_32, to_64));
///
/// assert_eq!(i8_to_i64(8i8), 8i64);
/// ```
///
/// # unstable
/// This function is 'unstable' because it uses nightly only unstable
/// features: [`unboxed_closures`] and [`fn_traits`] ([tracking issue])
///
/// This gives possibility to work with multi-argument functions
/// (see [second example])
///
/// See also:
/// - stable version of this function: [`fntools::chain`]
/// - extension on all functions: [`FnExtChain::chain`]
///
/// [`fn_traits`]: https://doc.rust-lang.org/unstable-book/library-features/fn-traits.html
/// [`unboxed_closures`]: https://doc.rust-lang.org/unstable-book/language-features/unboxed-closures.html
/// [tracking issue]: https://github.com/rust-lang/rust/issues/29625
/// [second example]: #second_example
/// [`FnExtChain::chain`]: crate::unstable::chain::FnExtChain::chain
/// [`fntools::chain`]: crate::chain
pub fn chain<A, C, F, G>(f: F, g: G) -> Chain<F, G, C>
where
    F: FnOnce<A>,
    G: FnOnce<C>,
    F::Output: AutoTuple<C>,
{
    Chain::new(f, g)
}

/// Represent composition of 2 functions `G ∘ F`
///
/// Note: `Chain` and [`Compose`] have no differences but argument order.
///
/// ## Why C?
/// `F` and `G` generic params are functions and `C` is args-type of `G`.
///
/// > Why `C` is here, but `A`  - not?
///
/// Because `A` is constrained in impl:
/// ```ignore
/// impl<A, C, F, G> FnOnce<A /* <-- */> for Chain<F, G, C>
/// where
///     F: FnOnce<A>,
///     F::Output: AutoTuple<C>,
///     G: FnOnce<C>,
/// ```
/// But `C` is not. To Fix this `C` was added to `Chain` struct.
///
/// [`Compose`]: crate::unstable::compose::compose
#[must_use = "function combinators are lazy and do nothing unless called"]
pub struct Chain<F, G, C>(F, G, PhantomData<dyn Fn(C)>);

impl<F, G, C> Chain<F, G, C> {
    pub fn new<A>(f: F, g: G) -> Self
    where
        F: FnOnce<A>,
        F::Output: AutoTuple<C>,
        G: FnOnce<C>,
    {
        Chain(f, g, PhantomData)
    }

    pub fn into_inner(self) -> (F, G) {
        let Chain(f, g, _) = self;
        (f, g)
    }
}

// F: A -> B
// G: C -> D
// B => C
//
// `B => C` needed for auto-tupling (needed for chaining _ -> (A, B) and (A, B) -> _)
impl<A, C, F, G> FnOnce<A> for Chain<F, G, C>
where
    F: FnOnce<A>,
    F::Output: AutoTuple<C>,
    G: FnOnce<C>,
{
    type Output = G::Output;

    #[allow(clippy::many_single_char_names)]
    extern "rust-call" fn call_once(self, args: A) -> Self::Output {
        let Chain(f, g, _) = self;
        let b: F::Output = f.call_once(args);
        let c: C = b.auto_tuple();
        let d: G::Output = g.call_once(c);
        d
    }
}

impl<A, C, F, G> FnMut<A> for Chain<F, G, C>
where
    F: FnMut<A>,
    F::Output: AutoTuple<C>,
    G: FnMut<C>,
{
    #[allow(clippy::many_single_char_names)]
    extern "rust-call" fn call_mut(&mut self, args: A) -> Self::Output {
        let Chain(f, g, _) = self;
        let b: F::Output = f.call_mut(args);
        let c: C = b.auto_tuple();
        let d: G::Output = g.call_mut(c);
        d
    }
}

impl<A, C, F, G> Fn<A> for Chain<F, G, C>
where
    F: Fn<A>,
    F::Output: AutoTuple<C>,
    G: Fn<C>,
{
    #[allow(clippy::many_single_char_names)]
    extern "rust-call" fn call(&self, args: A) -> Self::Output {
        let Chain(f, g, _) = self;
        let b: F::Output = f.call(args);
        let c: C = b.auto_tuple();
        let d: G::Output = g.call(c);
        d
    }
}

/// `.chain` extension for Fn* types
pub trait FnExtChain<A, B>: Sized {
    /// Chain two functions (`g ∘ self`)
    ///
    /// # Examples:
    /// ```
    /// // or `::unstable::fn_extensions::*`
    /// use fntools::unstable::chain::FnExtChain;
    ///
    /// let add_two = |a: i32| a + 2;
    /// let add_three = |a: i32| a + 3;
    /// let add_eight = add_two
    ///     .chain(add_three)
    ///     .chain(add_three);
    ///
    /// assert_eq!(add_eight(4), 12);
    /// ```
    ///
    /// For more info see [`chain`]
    ///
    /// [`chain`]: crate::unstable::chain::chain
    fn chain<C, G>(self, g: G) -> Chain<Self, G, C>
    where
        G: FnOnce<C>,
        B: AutoTuple<C>;
}

impl<A, F> FnExtChain<A, F::Output> for F
where
    F: FnOnce<A>,
{
    fn chain<C, G>(self, g: G) -> Chain<Self, G, C>
    where
        G: FnOnce<C>,
        F::Output: AutoTuple<C>,
    {
        chain(self, g)
    }
}

impl<F, G, C> Debug for Chain<F, G, C>
where
    F: Debug,
    G: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("Chain")
            .field("f", &self.0)
            .field("g", &self.1)
            .finish()
    }
}

impl<F, G, C> Clone for Chain<F, G, C>
where
    F: Clone,
    G: Clone,
{
    fn clone(&self) -> Self {
        Chain(self.0.clone(), self.1.clone(), PhantomData)
    }
}

impl<F, G, C> Copy for Chain<F, G, C>
where
    F: Copy,
    G: Copy,
{
}
