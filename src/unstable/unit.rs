/// Unit function output. (Analog to `chain(f, drop)`)
///
/// ## Examples
///
/// ```
/// use fntools::unstable::unit::unit;
///
/// let add = unit(|a: i32, b: i32| a + b);
///
/// assert_eq!(add(1, 2), ());
/// ```
#[inline]
pub fn unit<F, A>(f: F) -> Unit<F>
where
    F: FnOnce<A>,
{
    Unit::new(f)
}

/// Unit function output
///
/// See [`unit`](self::unit) for documentation
#[must_use = "function combinators are lazy and do nothing unless called"]
#[derive(Debug, Copy, Clone)]
pub struct Unit<F> {
    f: F,
}

impl<F> Unit<F> {
    #[inline]
    pub fn new<A>(f: F) -> Self
    where
        F: FnOnce<A>,
    {
        Self { f }
    }
}

impl<F, A> FnOnce<A> for Unit<F>
where
    F: FnOnce<A>,
{
    type Output = ();

    #[inline]
    extern "rust-call" fn call_once(self, args: A) -> Self::Output { self.f.call_once(args); }
}

impl<F, A> FnMut<A> for Unit<F>
where
    F: FnMut<A>,
{
    #[inline]
    extern "rust-call" fn call_mut(&mut self, args: A) -> Self::Output { self.f.call_mut(args); }
}

impl<F, A> Fn<A> for Unit<F>
where
    F: Fn<A>,
{
    #[inline]
    extern "rust-call" fn call(&self, args: A) -> Self::Output { self.f.call(args); }
}
