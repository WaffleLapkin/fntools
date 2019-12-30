/// Represents a type which can have functions applied to it (implemented
/// by default for all types).
pub trait ValueExt {
    /// Apply a function to `self`.
    ///
    /// # Examples
    /// ```
    /// use fntools::value::ValueExt;
    ///
    /// #[rustfmt::skip]
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
    /// # Examples
    /// ```
    /// use fntools::value::ValueExt;
    ///
    /// let mut also = 0;
    /// let val = (1 + 3)
    ///     .also(|it: &i32| assert_eq!(*it, 4))
    ///     .also(|it| also = it + 1);
    ///
    /// assert_eq!(val, 4);
    /// ```
    ///
    /// See also: [`also_mut`](self::ValueExt::also_mut)
    #[inline]
    fn also<F>(self, f: F) -> Self
    where
        Self: Sized,
        F: FnOnce(&Self) -> (),
    {
        f(&self);
        self
    }

    /// Execute function with unique reference to `self` and return `self`.
    ///
    /// # Examples
    /// ```
    /// use fntools::value::ValueExt;
    ///
    /// let val = 8
    ///     .also_mut(|it: &mut i32| assert_eq!(*it, 8))
    ///     .also_mut(|it| *it *= 2);
    ///
    /// assert_eq!(val, 16);
    /// ```
    ///
    /// See also: [`also`](self::ValueExt::also)
    #[inline]
    fn also_mut<F>(mut self, f: F) -> Self
    where
        Self: Sized,
        F: FnOnce(&mut Self) -> (),
    {
        f(&mut self);
        self
    }
}

// All functions of `ValueExt` actually require `Self: Sized` so `T: ?Sized`
// isn't currently needed, but it's placeholder for future.
impl<T: ?Sized> ValueExt for T {
    // use default definitions...
}
