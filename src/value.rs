/// Represents a type which can have functions applied to it (implemented
/// by default for all types).
pub trait ValueExt {
    /// Apply a function to `self`.
    ///
    /// # Examples
    /// ```
    /// use fntools::value::ValueExt;
    ///
    /// let val = (1, 4)
    ///     .apply(|(a, b)| a + b)
    ///     .apply(|it| it * it);
    ///
    /// assert_eq!(val, 25)
    /// ```
    #[inline]
    fn apply<F, R>(self, f: F) -> R
    where
        Self: Sized,
        F: FnOnce(Self) -> R,
    {
        f(self)
    }

    /// Execute function with reference to `self` and return `self`.
    ///
    /// Similar to [`dbg!`] macro - `dbg!(expression)` and
    /// `expression.also(|it| println!("{:?}", it))` do the same[^1] thing.
    ///
    /// # Examples
    /// ```
    /// use fntools::value::ValueExt;
    ///
    /// let mut also = 0;
    /// let val = (1 + 3)
    ///     .also(|it: &i32| println!("{}", it)) // will print 4
    ///     .also(|it| also = it + 1); // mutable state is not really needed here, just for example.
    ///
    /// assert_eq!(also, 5);
    /// assert_eq!(val, 4);
    /// ```
    /// [^1]: actually no, cause `dbg!` also prints file/line
    #[inline]
    fn also<F>(self, f: F) -> Self
    where
        Self: Sized,
        F: FnOnce(&Self) -> (),
    {
        f(&self);
        self
    }
}

// All functions of `ValueExt` actually require `Self: Sized` so `T: ?Sized`
// isn't currently needed, but it's placeholder for future.
impl<T: ?Sized> ValueExt for T {
    // use default definitions...
}
