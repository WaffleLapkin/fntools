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
pub struct Supply<T, F, A>(T, F, PhantomData<A>);

// TODO: for some reasons when param `A` moved to `new` (new<A>) type inference brakes
impl<T, F, A> Supply<T, F, A> {
    pub fn new(argument: T, f: F) -> Self
    where
        F: FnOnce<A>,
        A: TupleTake<Take = T>,
        A::Rem: TupleAppend<T, Res = A>,
    {
        Supply(argument, f, PhantomData)
    }
}

impl<T, E, F> FnOnce<E> for Supply<T, F, E::Res>
where
    F: FnOnce<E::Res>,
    E: TupleAppend<T>,
{
    type Output = F::Output;

    extern "rust-call" fn call_once(self, args: E) -> Self::Output {
        let Supply(data, f, _) = self;
        f.call_once(args.append(data))
    }
}

impl<T, E, F> FnMut<E> for Supply<T, F, E::Res>
where
    F: FnMut<E::Res>,
    E: TupleAppend<T>,
    T: Clone,
{
    extern "rust-call" fn call_mut(&mut self, args: E) -> Self::Output {
        let Supply(data, f, _) = self;
        f.call_mut(args.append(data.clone()))
    }
}

impl<T, E, F> Fn<E> for Supply<T, F, E::Res>
where
    F: Fn<E::Res>,
    E: TupleAppend<T>,
    T: Clone,
{
    extern "rust-call" fn call(&self, args: E) -> Self::Output {
        let Supply(data, f, _) = self;
        f.call(args.append(data.clone()))
    }
}

impl<T, F, A> Debug for Supply<T, F, A>
where
    T: Debug,
    F: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("Supply")
            .field("arg", &self.0)
            .field("f", &self.1)
            .finish()
    }
}

impl<T, F, A> Clone for Supply<T, F, A>
where
    T: Clone,
    F: Clone,
{
    fn clone(&self) -> Self {
        Supply(self.0.clone(), self.1.clone(), PhantomData)
    }
}

impl<T, F, A> Copy for Supply<T, F, A>
where
    T: Copy,
    F: Copy,
{
}
