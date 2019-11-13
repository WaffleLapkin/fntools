use crate::auto_tuple::AutoTuple;
use std::marker::PhantomData;
use std::fmt::{Debug, Formatter, Error};

/// Compose two functions.
///
/// Takes functions f and g and returns `f ∘ g` (in other words something
/// like `|a: A| f(g(a))`.
///
/// # Examples:
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
/// <a name="second_example"></a> `compose` also work with multi-argument
/// functions:
/// ```
/// use fntools::unstable::compose;
///
/// // very bad impl of `checked_add`
/// let my_checked_add =  compose(|res, over| if over { None } else { Some(res) }, i32::overflowing_add);
/// // note: no destructing needed ^^^^^^^^^                  return `(i32, bool)` ---- ^^^^^^^^^^^^^^^
/// assert_eq!(my_checked_add(8, 16), Some(24));
/// assert_eq!(my_checked_add(std::i32::MAX, 1), None);
/// ```
///
/// Note the order:
/// ```
/// use fntools::unstable::compose;
///
/// let to_16 = |i: i8| i16::from(i);
/// let to_32 = |i: i16| i32::from(i);
/// let to_64 = |i: i32| i64::from(i);
///
/// // execution order: to_16 -> to_32 -> to_64
/// let i8_to_i64 = compose(compose(to_64, to_32), to_16);
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
/// - stable version of this function: [`fntools::compose`]
/// - extension on all functions: [`FnExtCompose::compose`]
///
/// [`fn_traits`]: https://doc.rust-lang.org/unstable-book/library-features/fn-traits.html
/// [`unboxed_closures`]: https://doc.rust-lang.org/unstable-book/language-features/unboxed-closures.html
/// [tracking issue]: https://github.com/rust-lang/rust/issues/29625
/// [second example]: #second_example
/// [`FnExtCompose::compose`]: crate::unstable::compose::FnExtCompose::compose
/// [`fntools::compose`]: crate::compose
pub fn compose<A, B, C, D, F, G>(f: F, g: G) -> Compose<F, G, C>
where
    F: FnOnce<C, Output = D>,
    G: FnOnce<A, Output = B>,
    B: AutoTuple<C>,
{
    Compose::new(f, g)
}

/// Represent composition of 2 functions `F ∘ G`
///
/// Note: `Compose` and [`Chain`] have no differences but argument order.
///
/// ## Why C?
/// `F` and `G` generic params are functions and `C` is args-type of `G`.
///
/// > Why `C` is here, but `A`, `B`, `D` - not?
///
/// Because `A`, `B`, `D` are constrained parameters in impl:
/// ```ignore
/// impl<A, B, C, D, F, G> FnOnce<A> /* A constrained */ for Compose<F, G, C>
///     where
///         F: FnOnce<C, Output = D>, /* D constrained */
///         G: FnOnce<A, Output = B>, /* B constrained */
///         B: AutoTuple<C>,
/// ```
/// But `C` is not. To Fix this `C` was added to `Chain` struct.
///
/// [`Chain`]: crate::unstable::chain::Chain
#[must_use = "function combinators are lazy and do nothing unless called"]
pub struct Compose<F, G, C>(F, G, PhantomData<dyn Fn(C)>);

impl<F, G, C> Compose<F, G, C> {
    pub fn new<A, B, D>(f: F, g: G) -> Self
    where
        F: FnOnce<C, Output = D>,
        G: FnOnce<A, Output = B>,
        B: AutoTuple<C>,
    {
        Compose(f, g, PhantomData)
    }

    pub fn into_inner(self) -> (F, G) {
        let Compose(f, g, _) = self;
        (f, g)
    }
}

impl<A, B, C, D, F, G> FnOnce<A> for Compose<F, G, C>
where
    F: FnOnce<C, Output = D>,
    G: FnOnce<A, Output = B>,
    B: AutoTuple<C>,
{
    type Output = D;

    #[allow(clippy::many_single_char_names)]
    extern "rust-call" fn call_once(self, args: A) -> Self::Output {
        let Compose(f, g, _) = self;
        let b: B = g.call_once(args);
        let c: C = b.auto_tuple();
        let d: D = f.call_once(c);
        d
    }
}

impl<A, B, C, D, F, G> FnMut<A> for Compose<F, G, C>
where
    F: FnMut<C, Output = D>,
    G: FnMut<A, Output = B>,
    B: AutoTuple<C>,
{
    #[allow(clippy::many_single_char_names)]
    extern "rust-call" fn call_mut(&mut self, args: A) -> Self::Output {
        let Compose(f, g, _) = self;
        let b: B = g.call_mut(args);
        let c: C = b.auto_tuple();
        let d: D = f.call_mut(c);
        d
    }
}

impl<A, B, C, D, F, G> Fn<A> for Compose<F, G, C>
where
    F: Fn<C, Output = D>,
    G: Fn<A, Output = B>,
    B: AutoTuple<C>,
{
    #[allow(clippy::many_single_char_names)]
    extern "rust-call" fn call(&self, args: A) -> Self::Output {
        let Compose(f, g, _) = self;
        let b: B = g.call(args);
        let c: C = b.auto_tuple();
        let d: D = f.call(c);
        d
    }
}

/// `.compose` extension for Fn* types
pub trait FnExtCompose<C, D> {
    /// Compose two functions (`self ∘ g`)
    ///
    /// # Examples:
    /// ```
    /// // or `::unstable::fn_extensions::*`
    /// use fntools::unstable::compose::FnExtCompose;
    ///
    /// let add_two = |a: i32| a + 2;
    /// let add_three = |a: i32| a + 3;
    /// let add_eight = add_two
    ///     .compose(add_three)
    ///     .compose(add_three);
    ///
    /// assert_eq!(add_eight(4), 12);
    /// ```
    ///
    /// For more info see [`compose`]
    ///
    /// [`compose`]: crate::unstable::compose::compose
    fn compose<A, B, G>(self, g: G) -> Compose<Self, G, C>
    where
        Self: Sized,
        G: FnOnce<A, Output = B>,
        B: AutoTuple<C>;
}

impl<C, D, F> FnExtCompose<C, D> for F
where
    F: FnOnce<C, Output = D>,
{
    fn compose<A, B, G>(self, g: G) -> Compose<Self, G, C>
    where
        Self: Sized,
        G: FnOnce<A, Output = B>,
        B: AutoTuple<C>,
    {
        compose(self, g)
    }
}

impl<F, G, C> Debug for Compose<F, G, C>
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

impl<F, G, C> Clone for Compose<F, G, C>
where
    F: Clone,
    G: Clone,
{
    fn clone(&self) -> Self {
        Compose(self.0.clone(), self.1.clone(), PhantomData)
    }
}

impl<F, G, C> Copy for Compose<F, G, C>
where
    F: Copy,
    G: Copy,
{}
