/// Represents a type which can have functions applied to it (implemented
/// by default for all types).
pub trait ValueExtUnstable {
    /// Apply a function to `self` **u**n**t**upling `self`.
    ///
    /// # Examples
    /// ```
    /// use fntools::unstable::value::ValueExtUnstable;
    ///
    /// let val = (3, 4).apply_ut(|a, b| a * b);
    /// //                         ^^^^ ---- note: no destructing
    ///
    /// assert_eq!(val, 12)
    /// ```
    #[inline]
    fn apply_ut<F>(self, f: F) -> F::Output
    where
        Self: Sized,
        F: FnOnce<Self>,
    {
        f.call_once(self)
    }
}

// All functions of `ValueExtUnstable` actually require `Self: Sized` so `T:
// ?Sized` isn't currently needed, but it's placeholder for future.
impl<T: ?Sized> ValueExtUnstable for T {
    // use default definitions...
}
