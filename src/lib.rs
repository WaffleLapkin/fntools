//! ## Tools for working with functions
//! e.g.:
//! - chaining
//! - composing
//! - applying to values
//! - supplying arguments
//! - currying (O_O)
//! - Flipping arguments/output/both
//! - Cartesian product of functions
//! - Untupling (running a function `(A, B) -> _` on _argument_ `(A, B)`)
//!
//! ## DISCLAIMER
//! This library more an fun experiment with rust, than really useful library.
//!
//! However, in some cases it can make code a bit cleaner.
//!
//! ## Alternatives
//! Similar libraries:
//! - [tool](https://stebalien.github.io/tool-rs/tool/index.html)
//! - [compose](https://docs.rs/compose/0.1.0/compose/)
//!
//! You know other alternatives? - ping me in [telegram] or open issue on [github]!
//!
//! ## Stability
//! This library can work on both `stable` and `nightly` _however_ without
//! nightly it loses **a lot** of core functionality.
//!
//! To build on `stable` you need to add `"stable"` feature:
//! ```toml
//! // Cargo.toml
//! fntools = { version = "0.1.0", features = ["stable"] }
//! ```
//! This will remove [`unstable`] module with all the APIs which use
//! nightly-only unstable API.
//!
//! ## Unstable API
//! Unstable API provides these features:
//! - Multi-argument working (this uses a lot of hacks, but works!)
//!   + You can e.g. chain `(A, B) -> C` and `C -> D` to receive `(A, B) -> D`
//!   + You can e.g. chain `A -> (B, C)` and `(B, C) -> D`to receive `A -> D`
//!   + You can e.g. product `(A, B) -> C` and `X -> Y` to receive `(A, B, X) -> (C, Y)` // TODO
//! - Working with all fns at once (no `_mut` and `_once` versions of functions)
//! - Flipping big functions (e.g.: `(A, B, C) -> D` to `(C, B, A) -> D`) // TODO
//! - Destructing functions into inner functions (e.g.: [`Chain::into_inner`])
//! - Extensions on `Fn*` traits (e.g.: [`.chain`])
//!
//! Using [`unboxed_closures`] and [`fn_traits`] features ([tracking issue])
//!
//! ## See also
//! - [Wiki: Function Composition]
//! - [rossetacode.org: Function Composition]
//! - [stackoverflow: How to compose functions in Rust?]
//!
//! [telegram]: https://vee.gg/t/WaffleLapkin
//! [github]: https://github.com/WaffleLapkin/fntools
//!
//! [`unstable`]: crate::unstable
//!
//! [`Chain::into_inner`]: crate::unstable::chain::Chain::into_inner
//! [`.chain`]: crate::unstable::ext::FnExt::chain
//!
//! [`fn_traits`]: https://doc.rust-lang.org/unstable-book/library-features/fn-traits.html
//! [`unboxed_closures`]: https://doc.rust-lang.org/unstable-book/language-features/unboxed-closures.html
//! [tracking issue]: https://github.com/rust-lang/rust/issues/29625
//!
//! [Wiki: Function Composition]: https://en.wikipedia.org/wiki/Function_composition
//! [rossetacode.org: Function Composition]: https://rosettacode.org/wiki/Function_composition#Rust
//! [stackoverflow: How to compose functions in Rust?]: https://stackoverflow.com/questions/45786955/how-to-compose-functions-in-rust
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
    /// Flip tuple (`(A, B) => (B, A)`)
    pub mod flip;
    /// Pop element from tuple (`(A, B, T) => ((A, B), T)`)
    pub mod pop;
    /// Push element to tuple (`(A, B) + T => (A, B, T)`)
    pub mod push;
    /// Take element from tuple (`(T, A, B) => (T, (A, B))`)
    pub mod take;
    /// Provide traits for conversion `&(A, B) => (&A, &B)` and
    /// `&mut (A, B) => (&mut A, &mut B)`
    pub mod as_ref;
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
#[inline]
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
#[inline]
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
#[inline]
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
#[inline]
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
#[inline]
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
#[inline]
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
#[inline]
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
#[inline]
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
#[inline]
pub fn product_mut<A, B, X, Y, F, G>(mut f: F, mut g: G) -> impl FnMut(A, X) -> (B, Y)
where
    F: FnMut(A) -> B,
    G: FnMut(X) -> Y,
{
    move |a: A, x: X| (f(a), g(x))
}

mod sealed {
    pub trait Sealed {}

    impl Sealed for () {}
    impl<S: Sealed> Sealed for &'_ S {}
    impl<S: Sealed> Sealed for &'_ mut S {}

    macro_rules! tuple_impl {
        ($( $types:ident, )*) => {
            impl<$( $types, )*> Sealed for ($( $types, )*)
            where
                last_type!($( $types, )*): ?Sized,
            {}
        };
    }

    for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, # tuple_impl);
}
