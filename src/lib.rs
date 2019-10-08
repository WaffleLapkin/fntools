#![cfg_attr(not(stable), feature(unboxed_closures, fn_traits))]

#[cfg(not(stable))]
/// Features that uses nightly-only unstable API
pub mod unstable;

pub mod prelude {
    pub use super::{ValueExt, swap_args, chain, compose};
}

/// Represents a type which can have functions applied to it (implemented
/// by default for all types).
pub trait ValueExt {
    /// Apply a function which takes the parameter by value.
    /// 
    /// # Examples
    /// ```
    /// use fntools::ValueExt;
    /// 
    /// let val = (1, 4)
    ///     .apply(|(a, b)| a + b)
    ///     .apply(|it| it * it);
    /// 
    /// assert_eq!(val, 25)
    /// ```
    fn apply<R, F: FnOnce(Self) -> R>(self, f: F) -> R where Self: Sized {
        f(self)
    }

    /// Execute function with reference to `self` and return `self`.
    /// 
    /// Similar to [`dbg!`] macro - `dbg!(expression)` and `(expression).also(|it| println!("{:?}", it))` do the same[^1] thing.
    /// 
    /// # Examples
    /// ```
    /// use fntools::ValueExt;
    /// 
    /// let mut also = 0;
    /// let val = (1 + 3)
    ///     .also(|it: &i32| println!("{}", it)) // will print 4
    ///     .also(|it| also = it + 1); // mutable state is not really needed here, just for example.
    /// 
    /// assert_eq!(also, 5);
    /// assert_eq!(val, 4);
    /// ```
    /// [^1]: actually no, cause `dbg!` also prints file/line 
    fn also<F: FnOnce(&Self) -> ()>(self, f: F) -> Self where Self: Sized {
        f(&self);
        self
    }

    //fn
}

// All functions of `ValueExt` actually require `Self: Sized` so `T: ?Sized` isn't currently needed, but it's placeholder for future.
impl<T: ?Sized> ValueExt for T {
    // use default definitions...
}

/// Swap function arguments.
/// 
/// # Examples
/// ```
/// use fntools::swap_args;
/// 
/// let fun = |a: &str, b: i32| format!("{}{}", a, b);
/// let fun = swap_args(fun);
/// 
/// assert_eq!(fun(17, "hello, "), "hello, 17")
/// ```
pub fn swap_args<A, B, R, F>(f: F) -> impl FnOnce(B, A) -> R 
where F: FnOnce(A, B) -> R
{
    move |b: B, a: A| f(a, b)
}

/// Compose two functions.
///
/// Takes functions `f` and `g` and returns `f ∘ g = |a: A| f(g(a))`.
/// 
/// # Examples 
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
/// See also:
/// - [`unstable::compose`]
/// - [`fntools::chain`]
/// 
/// [`unstable::compose`]: crate::unstable::compose::compose
/// [`fntools::chain`]: crate::chain
pub fn compose<A, B, C, F, G>(f: F,  g: G) -> impl Fn(A) -> C
where 
    G: Fn(A) -> B,
    F: Fn(B) -> C,
{
    move |a: A| f(g(a))
}

/// Compose two functions.
///
/// Takes functions `f` and `g` and returns `g ∘ f = |a: A| g(f(a))`.
/// 
/// # Examples 
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
/// See also: 
/// - [`unstable::chain`]
/// - [`fntools::compose`]
/// 
/// [`unstable::chain`]: crate::unstable::chain::chain
/// [`fntools::compose`]: crate::compose
pub fn chain<A, B, C, F, G>(f: F,  g: G) -> impl Fn(A) -> C
where 
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |a: A| g(f(a))
}


pub fn constant<R>(val: R) -> impl FnOnce() -> R {
    move || val
}

pub fn constant_clone<R: Clone>(val: R) -> impl Fn() -> R {
    move || val.clone()
}