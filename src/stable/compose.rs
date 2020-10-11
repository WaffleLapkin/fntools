/// Compose two functions.
///
/// Takes functions `f` and `g` and returns `f ∘ g = |a: A| f(g(a))`.
///
/// # Examples
/// ```
/// use fntools::compose;
///
/// let add_two = |a: i32| a + 2;
/// let add_three = |a: i32| a + 3;
/// let add_five = compose(add_two, add_three);
///
/// assert_eq!(add_five(4), 9);
/// ```
///
/// Note the order:
/// ```
/// use fntools::compose;
///
/// let to_16 = |i: i8| i16::from(i);
/// let to_32 = |i: i16| i32::from(i);
/// let to_64 = |i: i32| i64::from(i);
///
/// // execution order: to_16 -> to_32 -> to_64
/// let i8_to_i64 = compose(compose(to_64, to_32), to_16);
///
/// assert_eq!(i8_to_i64(8i8), 8i64);
/// ```
///
/// See also:
/// - [`unstable::compose`]
/// - [`fntools::chain`]
///
/// [`unstable::compose`]: crate::unstable::compose
/// [`fntools::chain`]: crate::chain
#[inline]
pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    G: Fn(A) -> B,
    F: Fn(B) -> C,
{
    move |a: A| f(g(a))
}

/// Compose two functions which can be called only once.
///
/// See [compose](self::compose) for documentation.
#[inline]
pub fn compose_once<A, B, C, F, G>(f: F, g: G) -> impl FnOnce(A) -> C
where
    G: FnOnce(A) -> B,
    F: FnOnce(B) -> C,
{
    move |a: A| f(g(a))
}

/// Compose two functions which can be called only by unique reference.
///
/// See [compose](self::compose) for documentation.
#[inline]
pub fn compose_mut<A, B, C, F, G>(mut f: F, mut g: G) -> impl FnMut(A) -> C
where
    G: FnMut(A) -> B,
    F: FnMut(B) -> C,
{
    move |a: A| f(g(a))
}
