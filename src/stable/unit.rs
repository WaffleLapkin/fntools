/// Unit function output. Analog to `chain(chain, drop)`.
///
/// # Example
/// ```
/// use fntools::unit;
///
/// let fun = unit(|a: i32| -> i32 { a });
///
/// assert_eq!(fun(18), ());
/// ```
#[inline]
pub fn unit<A, B, F>(f: F) -> impl Fn(A) -> ()
where
    F: Fn(A) -> B,
{
    move |a: A| drop(f(a))
}

/// Unit function (which can be called only once) output.
///
/// See [unit](self::unit) for documentation.
#[inline]
pub fn unit_once<A, B, F>(f: F) -> impl FnOnce(A) -> ()
where
    F: FnOnce(A) -> B,
{
    move |a: A| drop(f(a))
}

/// Unit function (which can be called only by unique reference) output.
///
/// See [unit](self::unit) for documentation.
#[inline]
pub fn unit_mut<A, B, F>(mut f: F) -> impl FnMut(A) -> ()
where
    F: FnMut(A) -> B,
{
    move |a: A| drop(f(a))
}
