//! ## Tools for working with functions
//! e.g.:
//! - chaining
//! - composing
//! - applying to values
//! - supplying arguments
//! - currying (O_O)
//! - Flipping arguments/output/both
//! - Cartesian product of functions
//! - Untupling (running a function `A, B -> _` on _argument_ `(A, B)`)
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
//!   + You can e.g. chain `A, B -> C` and `C -> D` to receive `A, B -> D`
//!   + You can e.g. chain `A -> (B, C)` and `B, C -> D`to receive `A -> D`
//!   + You can e.g. product `A, B -> C` and `X -> Y` to receive `A, B, X -> (C, Y)` // TODO
//! - Working with all fns at once (no `_mut` and `_once` versions of functions)
//! - Flipping big functions (e.g.: `A, B, C -> D` to `C, B, A -> D`) // TODO
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
// TODO: find better place to store logo xD
#![doc(
    html_favicon_url = "https://cdn.discordapp.com/attachments/536882422848159784/648302525619109889/fxtestlogo.ico"
)]
#![doc(
    html_logo_url = "https://cdn.discordapp.com/attachments/536882422848159784/647078928044916746/logo_v0.1.4.min.svg"
)]

#[macro_use]
/// Helper macros these are used in this lib
mod local_macros;
/// Definitions on public (`#[macro_export]`) macros
mod macro_def;
/// 'Sealed' trait that prevents implementing tuple traits in other crates
mod sealed;

/// Helper module for moving stable thing to dedicated dir
mod stable {
    pub mod chain;
    pub mod compose;
    pub mod flip;
    pub mod product;
    /// Extensions for all types
    pub mod value;
}

pub use stable::{
    chain::{chain, chain_mut, chain_once},
    compose::{compose, compose_mut, compose_once},
    flip::{flip, flip_mut, flip_once},
    product::{product, product_mut, product_once},
    value,
};

/// Features that uses nightly-only unstable API
#[cfg(not(feature = "stable"))]
pub mod unstable;

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
    /// Concat tuples (`(A, B) + (C, D) => (A, B, C, D)`)
    ///
    /// **NOTE**: this module is under `#[cfg(feature = "concat")]`
    #[cfg(feature = "concat")]
    pub mod concat;
    /// Flip tuple (`(A, B) => (B, A)`)
    pub mod flip;
    /// Pop element from tuple (`(A, B, T) => ((A, B), T)`)
    pub mod pop;
    /// Push element to tuple (`(A, B) + T => (A, B, T)`)
    pub mod push;
    /// Take element from tuple (`(T, A, B) => (T, (A, B))`)
    pub mod take;
}
