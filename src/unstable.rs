/// 'Unstable' 'prelude'
/// 
/// # Examples
/// function extensions:
/// ```
/// use fntools::unstable::prelude::*;
/// 
/// let add_one = |it: i32| it + 1;
/// let add_two = |it: i32| it + 2;
/// let mul_seven = |it: i32| it * 7;
/// 
/// 
/// let res = mul_seven
///     .chain(add_two)
///     .compose(add_one)
///     (4);
/// 
/// assert_eq!(res, (4 + 1) * 7 + 2);
/// ```
pub mod prelude {
    pub use super::chain::{Chain, chain, FnExtChain};
    pub use super::compose::{Compose, compose, FnExtCompose};
}

pub use chain::{chain, Chain};
pub use compose::{compose, Compose};

pub mod chain {
    /// Chain two functions.
    ///
    /// Takes functions f and g and returns `g ∘ f` (in other words something like `|a: A| g(f(a))`.
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
    /// <a name="second_example"></a>chain also work with multi-argument functions:
    /// ```
    /// use fntools::unstable::chain;
    /// 
    /// let add = |a: i32, b: i32, c: i32| a + b + c;
    /// let sqr = |a: i32| a.pow(2);
    /// let fun = chain(add, sqr);
    /// assert_eq!(fun(1, 2, 3), 36)
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
    /// This function is 'unstable' because it uses nightly only unstable features: [`unboxed_closures`] and [`fn_traits`] ([tracking issue])
    /// 
    /// This gives possibility to work with multi-argument functions (see [second example])
    /// 
    /// See also: 
    /// - [`fntools::chain`]
    /// - [`FnExtChain::chain`]
    /// 
    /// [`fn_traits`]: https://doc.rust-lang.org/unstable-book/library-features/fn-traits.html
    /// [`unboxed_closures`]: https://doc.rust-lang.org/unstable-book/language-features/unboxed-closures.html
    /// [tracking issue]: https://github.com/rust-lang/rust/issues/29625
    /// [second example]: #second_example
    /// [`FnExtChain::chan`]: crate::unstable::chain::FnExtchain::chain
    /// [`fntools::chain`]: crate::chain
    pub fn chain<A, B, C, F, G>(f: F, g: G) -> Chain<F, G>
    where
        F: FnOnce<A, Output = B>,
        G: FnOnce<(B,), Output = C>,
    {
        Chain::new(f, g)
    }

    /// Represent composition of 2 functions `G ∘ F`
    /// 
    /// Note: `Chain` and [`Compose`] have no differences but argument order.
    /// 
    /// [`Compose`]: crate::unstable::compose::compose
    pub struct Chain<F, G>(F, G);

    impl<F, G> Chain<F, G> {
        pub fn new<A, B, C>(f: F, g: G) -> Self where 
            F: FnOnce<A, Output = B>,
            G: FnOnce<(B,), Output = C>,
        {
            Chain(f, g)
        }
    }

    impl<A, B, C, F, G> FnOnce<A> for Chain<F, G> 
    where 
        F: FnOnce<A, Output = B>,
        G: FnOnce<(B,), Output = C>,
    {
        type Output = C;

        extern "rust-call" fn call_once(self, args: A) -> Self::Output {
            let Chain(f, g) = self;
            let b: B = f.call_once(args);
            let c: C = g.call_once((b,));
            c
        }
    }

    impl<A, B, C, F, G> FnMut<A> for Chain<F, G> 
    where 
        F: FnMut<A, Output = B>,
        G: FnMut<(B,), Output = C>,
        //Self: FnOnce<A, Output = C>,
    {
        extern "rust-call" fn call_mut(&mut self, args: A) -> Self::Output {
            let Chain(f, g) = self;
            let b: B = f.call_mut(args);
            let c: C = g.call_mut((b,));
            c
        }
    }

    impl<A, B, C, F, G> Fn<A> for Chain<F, G> 
    where 
        F: Fn<A, Output = B>,
        G: Fn<(B,), Output = C>,
        //Self: FnMut<A, Output = C>,
    {
        extern "rust-call" fn call(&self, args: A) -> Self::Output {
            let Chain(f, g) = self;
            let b: B = f.call(args);
            let c: C = g.call((b,));
            c
        }
    }

    /// `.chain` extension for Fn* types
    pub trait FnExtChain<A, B> {
        /// Chain two functions (`g ∘ self`)
        /// 
        /// # Examples:
        /// ```
        /// use fntools::unstable::chain::FnExtChain; // or `::unstable::fn_extensions::*`
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
        fn chain<C, G>(self, g: G) -> Chain<Self, G> 
        where 
            Self: Sized, 
            G: FnOnce<(B,), Output = C>;
    }

    impl<A, B, F> FnExtChain<A, B> for F 
    where 
        F: FnOnce<A, Output = B>
    {
        fn chain<C, G>(self, g: G) -> Chain<Self, G> 
        where 
            Self: Sized, 
            G: FnOnce<(B,), Output = C> 
        {
            chain(self, g)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn as_mut<A, R, F>(f: F) -> impl FnMut<A, Output = R>
        where
            F: FnMut<A, Output = R>
        {
            f
        }

        #[test]
        fn mut_test() {
            let mut i = 0;
            let mut inc = |()| { i += 1; i };

            let mut print = |a: i32| println!("{}", a);
            let mut fun = as_mut(chain(print, inc));
            fun(1);
            fun(2);
        }
    }
}

pub mod compose {
    /// Represent composition of 2 functions `F ∘ G`
    /// 
    /// Note: `Compose` and [`Chain`] have no differences but argument order.
    /// 
    /// [`Chain`]: crate::unstable::chain::Chain
    pub struct Compose<F, G>(F, G);

    impl<F, G> Compose<F, G> {
        pub fn new<A, B, C>(f: F, g: G) -> Self where 
            F: FnOnce<(B,), Output = C>,
            G: FnOnce<A, Output = B>,
        {
            Compose(f, g)
        }
    }

    impl<A, B, C, F, G> FnOnce<A> for Compose<F, G> 
    where 
        F: FnOnce<(B,), Output = C>,
        G: FnOnce<A, Output = B>,
    {
        type Output = C;

        extern "rust-call" fn call_once(self, args: A) -> Self::Output {
            let Compose(f, g) = self;
            let b: B = g.call_once(args);
            let c: C = f.call_once((b,));
            c
        }
    }

    /// Compose two functions.
    ///
    /// Takes functions f and g and returns `f ∘ g` (in other words something like `|a: A| f(g(a))`.
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
    /// <a name="second_example"></a> chain also work with multi-argument functions:
    /// ```
    /// use fntools::unstable::chain;
    /// 
    /// let add = |a: i32, b: i32, c: i32| a + b + c;
    /// let sqr = |a: i32| a.pow(2);
    /// let fun = chain(add, sqr);
    /// assert_eq!(fun(1, 2, 3), 36)
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
    /// This function is 'unstable' because it uses nightly only unstable features: [`unboxed_closures`] and [`fn_traits`] ([tracking issue])
    /// 
    /// This gives possibility to work with multi-argument functions (see [second example])
    /// 
    /// See also: 
    /// - [`fntools::compose`]
    /// - [`FnExtCompose::compose`]
    ///
    /// [`fntools::compose`]: crate::compose
    /// [`fn_traits`]: https://doc.rust-lang.org/unstable-book/library-features/fn-traits.html
    /// [`unboxed_closures`]: https://doc.rust-lang.org/unstable-book/language-features/unboxed-closures.html
    /// [tracking issue]: https://github.com/rust-lang/rust/issues/29625
    /// [second example]: #second_example
    /// [`FnExtCompose::compose`]: crate::unstable::compose::FnExtCompose::compose
    pub fn compose<A, B, C, F, G>(f: F, g: G) -> Compose<F, G>
    where
        G: FnOnce<A, Output = B>,
        F: FnOnce<(B,), Output = C>,
    {
        Compose::new(f, g)
    }

    /// `.compose` extension for Fn* types
    pub trait FnExtCompose<B, C> {
        /// Compose two functions (`self ∘ g`)
        /// 
        /// # Examples:
        /// ```
        /// use fntools::unstable::compose::FnExtCompose; // or `::unstable::fn_extensions::*`
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
        fn compose<A, G>(self, g: G) -> Compose<Self, G> 
        where 
            Self: Sized, 
            G: FnOnce<A, Output = B>;
    }

    impl<B, C, F> FnExtCompose<B, C> for F 
    where 
        F: FnOnce<(B,), Output = C> 
    {
        fn compose<A, G>(self, g: G) -> Compose<Self, G> 
        where 
            Self: Sized, 
            G: FnOnce<A, Output = B>
        {
            compose(self, g)
        }
    }
}
