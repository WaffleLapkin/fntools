use crate::flip_tuple::FlipTuple;

/// Flip function output.
///
/// # Example
/// ```
/// use fntools::flip::flip_output;
///
/// /// see https://github.com/rust-lang/rust/issues/58052 for purposes of
/// /// this stupid function
/// fn assert_ty<T: FnOnce(&str) -> (&str, &str)>(t: T) -> T { t }
///
/// let fun = assert_ty(|a: &str| a.split_at(4));
/// let fun = flip_output(fun);
///
/// assert_eq!(fun("abcdefg"), ("efg", "abcd"))
/// ```
pub fn flip_output<A, B, C, F>(f: F) -> impl FnOnce(A) -> (C, B)
where
    F: FnOnce(A) -> (B, C)
{
    move |a: A| f(a).flip_tuple()
}
