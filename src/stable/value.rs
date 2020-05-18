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

    /// Execute function with unique reference to `self` and return `self`.
    ///
    /// # Examples
    /// ```
    /// use fntools::value::ValueExt;
    ///
    /// let val = 8
    ///     .also(|it: &mut i32| assert_eq!(*it, 8))
    ///     .also(|it| *it *= 2);
    ///
    /// assert_eq!(val, 16);
    /// ```
    #[inline]
    fn also<F>(mut self, f: F) -> Self
    where
        Self: Sized,
        F: FnOnce(&mut Self) -> (),
    {
        f(&mut self);
        self
    }

    /// Execute function with unique reference to deref-target of `self` and
    /// return `self`.
    ///
    /// This may be useful when you want to use `.also(T::fun)` on value of type
    /// `U` where `U: DerefMut<Target = T>` (e.g.: `Vec`+`[_]`,
    /// `String`+`str`, `Box<T>`+`T`)
    ///
    /// ## Examples
    ///
    /// ```
    /// use fntools::value::ValueExt;
    ///
    /// let val = vec![2, 1].also_deref(<[_]>::sort);
    /// assert_eq!(val, [1, 2]);
    ///
    /// let what = String::from("what?").also_deref(str::make_ascii_uppercase);
    /// assert_eq!(what, "WHAT?");
    ///
    /// let boxed = Box::<str>::from("BOXED").also_deref(str::make_ascii_lowercase);
    /// assert_eq!(&*boxed, "boxed");
    /// ```
    #[inline]
    fn also_deref<F>(mut self, f: F) -> Self
    where
        Self: Sized + core::ops::DerefMut,
        F: FnOnce(&mut Self::Target) -> (),
    {
        f(self.deref_mut());
        self
    }
}

// All functions of `ValueExt` actually require `Self: Sized` so `T: ?Sized`
// isn't currently needed, but it's placeholder for future.
impl<T: ?Sized> ValueExt for T {
    // use default definitions...
}
