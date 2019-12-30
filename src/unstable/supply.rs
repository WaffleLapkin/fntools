use std::{
    fmt::{Debug, Error, Formatter},
    marker::PhantomData,
};

use crate::tuple::{append::TupleAppend, take::TupleTake};

/// Supply argument to function
///
/// ## Examples
/// ```
/// use fntools::unstable::supply::supply;
///
/// let add = |a: i32, b: i32| a + b;
/// let add_5 = supply(5, add);
///
/// assert_eq!(add_5(5), 10);
/// assert_eq!(add_5(661), 666);
/// ```
/// ```
/// use fntools::unstable::supply::supply;
///
/// let fun = |a: i32, b: usize, c: String| format!("a: {}, b: {}, c: {:?}", a, b, c);
/// let fun = supply(8, fun);
/// let fun = supply(16, fun);
/// let fun = supply(String::from("AAA"), fun);
///
/// assert_eq!(fun(), "a: 8, b: 16, c: \"AAA\"")
/// ```
#[inline]
pub fn supply<F, A>(argument: A::Take, f: F) -> Supply<A::Take, F, A>
where
    F: FnOnce<A>,
    A: TupleTake,
{
    Supply::new(argument, f)
}

/// Represent supplying param `T` to function `F`
///
/// See [supply](self::supply) for documentation
#[must_use = "function combinators are lazy and do nothing unless called"]
pub struct Supply<T, F, A> {
    /// Supplied argument
    argument: T,
    f: F,
    marker: PhantomData<dyn Fn(A)>,
}

// TODO: for some reasons when param `A` moved to `new` (new<A>)
//   type inference brakes
impl<T, F, A> Supply<T, F, A> {
    #[inline]
    pub fn new(argument: T, f: F) -> Self
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
