/// Chain two functions.
///
/// Takes functions `f` and `g` and returns `g ∘ f = |a: A| g(f(a))`.
///
/// # Examples
/// ```
/// use fntools::chain;
///
/// let add_two = |a: i32| a + 2;
/// let add_three = |a: i32| a + 3;
/// let add_five = chain(add_two, add_three);
///
/// assert_eq!(add_five(4), 9);
/// ```
///
/// Note the order:
/// ```
/// use fntools::chain;
///
/// let to_16 = |i: i8| i16::from(i);
/// let to_32 = |i: i16| i32::from(i);
/// let to_64 = |i: i32| i64::from(i);
///
/// // execution order: to_16 -> to_32 -> to_64
/// let i8_to_i64 = chain(to_16, chain(to_32, to_64));
///
/// assert_eq!(i8_to_i64(8i8), 8i64);
/// ```
///
/// See also:
/// - [`unstable::chain`]
/// - [`fntools::compose`]
///
/// [`unstable::chain`]: crate::unstable::chain::chain
/// [`fntools::compose`]: crate::compose
#[inline]
pub fn chain<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |a: A| g(f(a))
}

/// Chain two functions which can be called only once.
///
/// See [chain](self::chain) for documentation.
#[inline]
pub fn chain_once<A, B, C, F, G>(f: F, g: G) -> impl FnOnce(A) -> C
where
    F: FnOnce(A) -> B,
    G: FnOnce(B) -> C,
{
    move |a: A| g(f(a))
}

/// Chain two functions which can be called only by unique reference.
///
/// See [chain](self::chain) for documentation.
#[inline]
pub fn chain_mut<A, B, C, F, G>(mut f: F, mut g: G) -> impl FnMut(A) -> C
where
    F: FnMut(A) -> B,
    G: FnMut(B) -> C,
{
    move |a: A| g(f(a))
}
