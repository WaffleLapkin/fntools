use crate::tuple::flip::FlipTuple;

/// Flip both function arguments and output.
///
/// # Example
/// ```
/// use fntools::flip::flip_full;
///
/// /// see https://github.com/rust-lang/rust/issues/58052 for purposes of
/// /// this stupid function
/// fn assert_ty<T: Fn(&str, usize) -> (&str, &str)>(t: T) -> T { t }
///
/// let fun = assert_ty(|a: &str, b: usize| -> (&str, &str) { a.split_at(b) });
/// let fun = flip_full(fun);
///
/// assert_eq!(fun(4, "hello, "), ("o, ", "hell"))
/// ```
pub fn flip_full<A, B, C, D, F>(f: F) -> impl Fn(B, A) -> (D, C)
where
    F: Fn(A, B) -> (C, D),
{
    move |b: B, a: A| f(a, b).flip_tuple()
}

/// Flip both function (which can be called only once) arguments and output.
///
/// See [flip_full](self::flip_full) for documentation.
pub fn flip_full_once<A, B, C, D, F>(f: F) -> impl FnOnce(B, A) -> (D, C)
where
    F: FnOnce(A, B) -> (C, D),
{
    move |b: B, a: A| f(a, b).flip_tuple()
}

/// Flip both function (which can be called only by unique reference) arguments
/// and output.
///
/// See [flip_full](self::flip_full) for documentation.
pub fn flip_full_mut<A, B, C, D, F>(mut f: F) -> impl FnMut(B, A) -> (D, C)
where
    F: FnMut(A, B) -> (C, D),
{
    move |b: B, a: A| f(a, b).flip_tuple()
}
