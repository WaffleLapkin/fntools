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

pub mod chain;
pub mod compose;
