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
