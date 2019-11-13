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
pub fn flip_args<A, B, R, F>(f: F) -> impl Fn(B, A) -> R
where
    F: Fn(A, B) -> R,
{
    move |b: B, a: A| f(a, b)
}

/// Flip function (which can be called only once) arguments.
///
/// See [flip_args](self::flip_args) for documentation.
pub fn flip_args_once<A, B, R, F>(f: F) -> impl FnOnce(B, A) -> R
where
    F: FnOnce(A, B) -> R,
{
    move |b: B, a: A| f(a, b)
}

/// Flip function (which can be called only by unique reference) arguments.
///
/// See [flip_args](self::flip_args) for documentation.
pub fn flip_args_mut<A, B, R, F>(mut f: F) -> impl FnMut(B, A) -> R
where
    F: FnMut(A, B) -> R,
{
    move |b: B, a: A| f(a, b)
}
