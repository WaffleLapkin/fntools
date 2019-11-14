use crate::tuple::flip::FlipTuple;

/// Flip function output.
///
/// # Example
/// ```
/// use fntools::flip::flip_output;
///
/// /// see https://github.com/rust-lang/rust/issues/58052 for purposes of
/// /// this stupid function
/// fn assert_ty<T: Fn(&str) -> (&str, &str)>(t: T) -> T { t }
///
/// let fun = assert_ty(|a: &str| a.split_at(4));
/// let fun = flip_output(fun);
///
/// assert_eq!(fun("abcdefg"), ("efg", "abcd"))
/// ```
pub fn flip_output<A, B, C, F>(f: F) -> impl Fn(A) -> (C, B)
where
    F: Fn(A) -> (B, C),
{
    move |a: A| f(a).flip_tuple()
}

/// Flip function (which can be called only once) output.
///
/// See [flip_output](self::flip_output) for documentation.
pub fn flip_output_once<A, B, C, F>(f: F) -> impl FnOnce(A) -> (C, B)
where
    F: FnOnce(A) -> (B, C),
{
    move |a: A| f(a).flip_tuple()
}

/// Flip function (which can be called only by unique reference) output.
///
/// See [flip_output](self::flip_output) for documentation.
pub fn flip_output_mut<A, B, C, F>(mut f: F) -> impl FnMut(A) -> (C, B)
where
    F: FnMut(A) -> (B, C),
{
    move |a: A| f(a).flip_tuple()
}
