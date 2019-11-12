/// Return function that return constant.
///
/// # Example
/// ```
/// use fntools::constant;
///
/// let fun = constant(String::from("Hi"));
/// assert_eq!(fun(), "Hi");
/// ```
pub fn constant<R>(val: R) -> impl FnOnce() -> R {
    move || val
}

/// Return function that repeatedly return constant.
///
/// # Example
/// ```
/// use fntools::constant::constant_clone;
///
/// let fun = constant_clone(String::from("Hi"));
/// assert_eq!(fun(), "Hi"); // Note: `.clone` call
/// assert_eq!(fun(), "Hi"); // Note: `.clone` call
/// assert_eq!(fun(), "Hi"); // Note: `.clone` call
/// ```
pub fn constant_clone<R>(val: R) -> impl Fn() -> R
    where
        R: Clone,
{
    move || val.clone()
}

/// Return function that repeatedly return constant.
///
/// # Example
/// ```
/// use fntools::constant::constant_copy;
///
/// let fun = constant_copy(42);
/// assert_eq!(fun(), 42);
/// assert_eq!(fun(), 42);
/// assert_eq!(fun(), 42);
/// ```
pub fn constant_copy<'a, R>(val: R) -> impl Fn() -> R
    where
        R: Copy,
{
    move || val
}

/// Return function that repeatedly return constant.
///
/// # Example
/// ```
/// use fntools::constant::constant_clone_ref;
///
/// let string = String::from("Hi");
/// let fun = constant_clone_ref(&string);
/// assert_eq!(fun(), "Hi"); // Note: `.clone` call
/// assert_eq!(fun(), "Hi"); // Note: `.clone` call
/// assert_eq!(fun(), "Hi"); // Note: `.clone` call
/// ```
pub fn constant_clone_ref<'a, R>(val: &'a R) -> impl Fn() -> R + 'a
    where
        R: Clone,
{
    move || val.clone()
}
