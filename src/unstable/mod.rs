pub mod prelude {
    pub use super::{chain::chain, compose::compose, ext::FnExt, supply::supply, untuple::untuple};
}

pub mod chain;
pub mod compose;
pub mod curry;
pub mod ext;
pub mod flip;
pub mod supply;
pub mod untuple;
pub mod value;
