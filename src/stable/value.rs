use std::ops::{Deref, DerefMut};

/// Represents a type which can have functions applied to it (implemented
/// by default for all types).
pub trait Apply {
    /// Apply a function to `self`.
    ///
    /// i.e. literally `f(self)`.
    ///
    /// ## Examples
    ///
    /// ```
    /// use fntools::value::Apply;
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

    /// Apply a function to a reference to `self`.
    ///
    /// i.e. literally `f(&self)`.
    ///
    /// ## Examples
    ///
    /// ```
    /// use fntools::value::Apply;
    ///
    /// let val = [0, 1, 2][..].apply_ref(<[_]>::len);
    /// assert_eq!(val, 3)
    /// ```
    #[inline]
    fn apply_ref<'a, F, R>(&'a self, f: F) -> R
    where
        F: FnOnce(&'a Self) -> R,
    {
        f(self)
    }

    /// Apply a function to a unique reference to `self`.
    ///
    /// i.e. literally `f(&mut self)`.
    ///
    /// ## Examples
    ///
    /// ```
    /// use fntools::value::Apply;
    /// use std::mem::replace;
    ///
    /// let mut arr = [0, 1, 2];
    /// let val = arr.apply_mut(|[a, _, c]| replace(a, 42) + replace(c, 4));
    ///
    /// assert_eq!(val, 2);
    /// assert_eq!(arr, [42, 1, 4]);
    /// ```
    #[inline]
    fn apply_mut<'a, F, R>(&'a mut self, f: F) -> R
    where
        F: FnOnce(&'a mut Self) -> R,
    {
        f(self)
    }

    /// Apply a function to a deref of `self`.
    ///
    /// i.e. literally `f(&*self)`.
    ///
    /// ## Examples
    ///
    /// ```
    /// use fntools::value::Apply;
    ///
    /// let val = vec![13748, 3738, 10003, 17].apply_deref(<[_]>::len);
    ///
    /// assert_eq!(val, 4);
    /// ```
    #[inline]
    // https://github.com/rust-lang/rust-clippy/issues/6159
    #[allow(clippy::needless_lifetimes)]
    fn apply_deref<'a, F, R>(&'a self, f: F) -> R
    where
        Self: Deref,
        F: FnOnce(&'a Self::Target) -> R,
    {
        self.deref().apply(f)
    }

    /// Apply a function to a mut deref of `self`.
    ///
    /// i.e. literally `f(&mut *self)`.
    ///
    /// ## Examples
    ///
    /// ```
    /// use fntools::value::Apply;
    ///
    /// let mut vec = vec![13748, 3738, 10003, 17];
    /// let () = vec.apply_deref_mut(|s| s.copy_from_slice(&[0; 4]));
    ///
    /// assert_eq!(vec, [0, 0, 0, 0]);
    /// ```
    #[inline]
    // https://github.com/rust-lang/rust-clippy/issues/6159
    #[allow(clippy::needless_lifetimes)]
    fn apply_deref_mut<'a, F, R>(&'a mut self, f: F) -> R
    where
        Self: DerefMut,
        F: FnOnce(&'a mut Self::Target) -> R,
    {
        self.deref_mut().apply(f)
    }
}

impl<T: ?Sized> Apply for T {
    // use default definitions...
}

/// Represents a type which can have functions applied to it (implemented
/// by default for all types).
pub trait Also: Sized {
    /// Execute function with reference to `self` and return `self`.
    ///
    /// This may be usefull when you want to make some side effect.
    ///
    /// # Examples
    /// ```
    /// use fntools::value::Also;
    ///
    /// #[rustfmt::skip]
    /// let val = 8
    ///     .also(|it: &i32| assert_eq!(*it, 8))
    ///     .also(|it| { dbg!(it); });
    ///
    /// assert_eq!(val, 8);
    /// ```
    #[inline]
    fn also<F>(self, f: F) -> Self
    where
        F: FnOnce(&Self) -> (),
    {
        self.apply_ref(f);
        self
    }

    /// Execute function with unique reference to `self` and return `self`.
    ///
    /// This may be usefull when you want to make some side effect and/or mutate
    /// `self`.
    ///
    /// # Examples
    /// ```
    /// use fntools::value::Also;
    ///
    /// let val = 8
    ///     .also_mut(|it: &mut i32| assert_eq!(*it, 8))
    ///     .also_mut(|it| *it *= 2);
    ///
    /// assert_eq!(val, 16);
    /// ```
    #[inline]
    fn also_mut<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut Self) -> (),
    {
        self.apply_mut(f);
        self
    }

    /// Execute function with reference to deref-target of `self` and
    /// return `self`.
    ///
    /// This may be useful when you want to use `.also(T::fun)` on value of type
    /// `U` where `U: Deref<Target = T>` (e.g.: `Vec`+`[_]`,
    /// `String`+`str`, `Box<T>`+`T`)
    ///
    /// ## Examples
    ///
    /// ```
    /// use fntools::value::Also;
    ///
    /// let mut out = None;
    /// let val = vec![2, 17, 1, 3, 5].also_deref(|s| out = Some(<[_]>::len(s)));
    ///
    /// assert_eq!(out, Some(5));
    /// assert_eq!(val, [2, 17, 1, 3, 5]);
    /// ```
    #[inline]
    fn also_deref<F>(self, f: F) -> Self
    where
        Self: Deref,
        F: FnOnce(&Self::Target) -> (),
    {
        self.apply_deref(f);
        self
    }

    /// Execute function with unique reference to deref-target of `self` and
    /// return `self`.
    ///
    /// This may be useful when you want to use `.also_mut(T::fun)` on value of
    /// type `U` where `U: DerefMut<Target = T>` (e.g.: `Vec`+`[_]`,
    /// `String`+`str`, `Box<T>`+`T`)
    ///
    /// ## Examples
    ///
    /// ```
    /// use fntools::value::Also;
    ///
    /// let val = vec![2, 17, 1, 3, 5].also_deref_mut(<[_]>::sort);
    /// assert_eq!(val, [1, 2, 3, 5, 17]);
    ///
    /// let what = String::from("what?").also_deref_mut(str::make_ascii_uppercase);
    /// assert_eq!(what, "WHAT?");
    ///
    /// let boxed = Box::<str>::from("BOXED").also_deref_mut(str::make_ascii_lowercase);
    /// assert_eq!(&*boxed, "boxed");
    /// ```
    #[inline]
    fn also_deref_mut<F>(mut self, f: F) -> Self
    where
        Self: DerefMut,
        F: FnOnce(&mut Self::Target) -> (),
    {
        self.apply_deref_mut(f);
        self
    }
}

impl<T> Also for T {
    // use default definitions...
}
