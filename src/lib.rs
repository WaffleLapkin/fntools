#![cfg_attr(not(feature = "stable"), feature(unboxed_closures, fn_traits))]

#[macro_use]
mod local_macros;

/// Flipping function arguments/output/both
pub mod flip;
/// Features that uses nightly-only unstable API
#[cfg(not(feature = "stable"))]
pub mod unstable;
/// Extension for all types
pub mod value;

/// Helpers for working with tuples
///
/// **Note**: in all of the traits there is no tuples of arity >= 13 (neither in requirements
/// neither in return types). It's because Rust current type system can't express "tuple of any
/// size" (see [Draft RFC: variadic generics] for proposes how to fix this) so this lib follows the
/// [stdlib] in implementing traits on tuples of arity 12 or less.
///
/// [Draft RFC: variadic generics]: https://github.com/rust-lang/rfcs/issues/376
/// [stdlib]: https://doc.rust-lang.org/std/primitive.tuple.html#trait-implementations
pub mod tuple {
    /// Append element to tuple (`T + (A, B) => (T, A, B)`)
    pub mod append;
    /// Convert everything to tuples (`A => (A,)`; `(A, B) => (A, B)`)
    pub mod auto_tuple;
    /// Flip tuple (`(A, B) => (B, A)`)
    pub mod flip;
    /// Pop element from tuple (`(A, B, T) => ((A, B), T)`)
    pub mod pop;
    /// Push element to tuple (`(A, B) + T => (A, B, T)`)
    pub mod push;
    /// Take element from tuple (`(T, A, B) => (T, (A, B))`)
    pub mod take;
}

pub mod prelude {
    pub use crate::{chain, value::ValueExt};
}

mod macro_def;

/// Compose two functions.
///
/// Takes functions `f` and `g` and returns `f ∘ g = |a: A| f(g(a))`.
///
/// # Examples
/// ```
/// use fntools::compose;
///
/// let add_two = |a: i32| a + 2;
/// let add_three = |a: i32| a + 3;
/// let add_five = compose(add_two, add_three);
///
/// assert_eq!(add_five(4), 9);
/// ```
///
/// Note the order:
/// ```
/// use fntools::compose;
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
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    G: Fn(A) -> B,
    F: Fn(B) -> C,
{
    move |a: A| f(g(a))
}

/// Compose two functions which can be called only once.
///
/// See [compose](self::compose) for documentation.
pub fn compose_once<A, B, C, F, G>(f: F, g: G) -> impl FnOnce(A) -> C
where
    G: FnOnce(A) -> B,
    F: FnOnce(B) -> C,
{
    move |a: A| f(g(a))
}

/// Compose two functions which can be called only by unique reference.
///
/// See [compose](self::compose) for documentation.
pub fn compose_mut<A, B, C, F, G>(mut f: F, mut g: G) -> impl FnMut(A) -> C
where
    G: FnMut(A) -> B,
    F: FnMut(B) -> C,
{
    move |a: A| f(g(a))
}

/// Chain two functions.
///
/// Takes functions `f` and `g` and returns `g ∘ f = |a: A| g(f(a))`.
///
/// # Examples
/// ```
/// use fntools::chain;
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
/// use fntools::chain;
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
pub fn chain<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |a: A| g(f(a))
}

/// Chain two functions which can be called only once.
///
/// See [chain](self::chain) for documentation.
pub fn chain_once<A, B, C, F, G>(f: F, g: G) -> impl FnOnce(A) -> C
where
    F: FnOnce(A) -> B,
    G: FnOnce(B) -> C,
{
    move |a: A| g(f(a))
}

/// Chain two functions which can be called only by unique reference.
///
/// See [chain](self::chain) for documentation.
pub fn chain_mut<A, B, C, F, G>(mut f: F, mut g: G) -> impl FnMut(A) -> C
where
    F: FnMut(A) -> B,
    G: FnMut(B) -> C,
{
    move |a: A| g(f(a))
}

/// Cartesian product of functions.
///
/// Takes functions `f` and `g` and returns `g × f = |a: A, x: X| (f(a), g(x))`.
///
/// ## Example
/// ```
/// use fntools::product;
///
/// // TODO: better example
/// let string = "привет";
/// let (slice, str) = product(<[_]>::len, str::len)(string.as_bytes(), string);
/// assert_eq!(slice, 12);
/// assert_eq!(str, 12);
/// ```
pub fn product<A, B, X, Y, F, G>(f: F, g: G) -> impl Fn(A, X) -> (B, Y)
where
    F: Fn(A) -> B,
    G: Fn(X) -> Y,
{
    move |a: A, x: X| (f(a), g(x))
}

/// Cartesian product of functions which can be called only once.
///
/// See [product](self::product) for documentation.
pub fn product_once<A, B, X, Y, F, G>(f: F, g: G) -> impl FnOnce(A, X) -> (B, Y)
where
    F: FnOnce(A) -> B,
    G: FnOnce(X) -> Y,
{
    move |a: A, x: X| (f(a), g(x))
}

/// Cartesian product of functions which can be called only by unique reference.
///
/// See [product](self::product) for documentation.
pub fn product_mut<A, B, X, Y, F, G>(mut f: F, mut g: G) -> impl FnMut(A, X) -> (B, Y)
where
    F: FnMut(A) -> B,
    G: FnMut(X) -> Y,
{
    move |a: A, x: X| (f(a), g(x))
}
