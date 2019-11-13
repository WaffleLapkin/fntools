use crate::flip_tuple::FlipTuple;

/// Flip both function arguments and output.
///
/// # Example
/// ```
/// use fntools::flip::flip_full;
///
/// /// see https://github.com/rust-lang/rust/issues/58052 for purposes of
/// /// this stupid function
/// fn assert_ty<T: FnOnce(&str, usize) -> (&str, &str)>(t: T) -> T { t }
///
/// let fun = assert_ty(|a: &str, b: usize| -> (&str, &str) { a.split_at(b) });
/// let fun = flip_full(fun);
///
/// assert_eq!(fun(4, "hello, "), ("o, ", "hell"))
/// ```
pub fn flip_full<A, B, C, D, F>(f: F) -> impl FnOnce(B, A) -> (D, C)
where
    F: FnOnce(A, B) -> (C, D)
{
    move |b: B, a: A| f(a, b).flip_tuple()
}
