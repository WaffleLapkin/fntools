use std::{
    fmt::{Debug, Error, Formatter},
    marker::PhantomData,
};

use crate::tuple::{append::TupleAppend, take::TupleTake};

/// Supply `argument` to the function `f`.
///
/// ## Examples
/// ```
/// use fntools::unstable::supply;
///
/// let add = |a: i32, b: i32| a + b;
/// let add_5 = supply(add, 5);
///
/// assert_eq!(add_5(5), 10);
/// assert_eq!(add_5(661), 666);
/// ```
/// ```
/// use fntools::unstable::supply;
///
/// let fun = |a: i32, b: usize, c: String| format!("a: {}, b: {}, c: {:?}", a, b, c);
/// let fun = supply(fun, 8);
/// let fun = supply(fun, 16);
/// let fun = supply(fun, String::from("AAA"));
///
/// assert_eq!(fun(), "a: 8, b: 16, c: \"AAA\"");
/// ```
#[inline]
pub fn supply<F, A>(f: F, argument: A::Take) -> Supply<A::Take, F, A>
where
    F: FnOnce<A>,
    A: TupleTake,
{
    Supply::new(f, argument)
}

/// Represents function `F` with supplied argument `T`.
///
/// See [`supply`] for documentation.
#[must_use = "function combinators are lazy and do nothing unless called"]
pub struct Supply<T, F, A> {
    argument: T,
    f: F,
    marker: PhantomData<fn(A)>,
}

// FIXME(waffle): for some reasons when param `A` moved to `new` (new<A>)
//                type inference brakes
impl<T, F, A> Supply<T, F, A> {
    /// Creates version of the functions `f` with supplied `argument`.
    ///
    /// It's preferred to use [`supply`] instead.
    #[inline]
    pub fn new(f: F, argument: T) -> Self
    where
        F: FnOnce<A>,
        A: TupleTake<Take = T>,
        A::Rem: TupleAppend<T, Res = A>,
    {
        Supply {
            argument,
            f,
            marker: PhantomData,
        }
    }

    /// Returns inner function and supplied argument.
    #[inline]
    pub fn into_inner(self) -> (F, T) {
        let Supply {
            f,
            argument,
            marker: _,
        } = self;
        (f, argument)
    }

    /// Returns references to function and supplied argument.
    #[inline]
    pub fn as_inner(&self) -> (&F, &T) {
        let Supply {
            f,
            argument,
            marker: _,
        } = self;
        (f, argument)
    }
}

impl<T, E, F> FnOnce<E> for Supply<T, F, E::Res>
where
    F: FnOnce<E::Res>,
    E: TupleAppend<T>,
{
    type Output = F::Output;

    #[inline]
    extern "rust-call" fn call_once(self, args: E) -> Self::Output {
        let Supply { argument, f, .. } = self;
        f.call_once(args.append(argument))
    }
}

impl<T, E, F> FnMut<E> for Supply<T, F, E::Res>
where
    F: FnMut<E::Res>,
    E: TupleAppend<T>,
    T: Clone,
{
    #[inline]
    extern "rust-call" fn call_mut(&mut self, args: E) -> Self::Output {
        let Supply { argument, f, .. } = self;
        f.call_mut(args.append(argument.clone()))
    }
}

impl<T, E, F> Fn<E> for Supply<T, F, E::Res>
where
    F: Fn<E::Res>,
    E: TupleAppend<T>,
    T: Clone,
{
    #[inline]
    extern "rust-call" fn call(&self, args: E) -> Self::Output {
        let Supply { argument, f, .. } = self;
        f.call(args.append(argument.clone()))
    }
}

impl<T, F, A> Debug for Supply<T, F, A>
where
    T: Debug,
    F: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("Supply")
            .field("argument", &self.argument)
            .field("f", &self.f)
            .finish()
    }
}

impl<T, F, A> Clone for Supply<T, F, A>
where
    T: Clone,
    F: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Supply {
            argument: self.argument.clone(),
            f: self.f.clone(),
            marker: PhantomData,
        }
    }
}

impl<T, F, A> Copy for Supply<T, F, A>
where
    T: Copy,
    F: Copy,
{
}
