/// Flip function arguments.
///
/// # Example
/// ```
/// use fntools::flip::flip_args;
///
/// let fun = |a: &str, b: i32| format!("{}{}", a, b);
/// let fun = flip_args(fun);
///
/// assert_eq!(fun(17, "hello, "), "hello, 17")
/// ```
pub fn flip_args<A, B, R, F>(f: F) -> impl FnOnce(B, A) -> R
where
    F: FnOnce(A, B) -> R
{
    move |b: B, a: A| f(a, b)
}
