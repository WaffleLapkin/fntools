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
pub fn supply<T, F, A>(argument: T, f: F) -> Supply<T, F, A>
where
    F: FnOnce<A>,
    A: TupleTake<Take = T>,
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

/// `.supply` extension for Fn* types
pub trait FnExtSupply<T, A>: Sized {
    /// Chain two functions (`g ∘ self`)
    ///
    /// # Examples:
    /// ```
    /// // or `::unstable::fn_extensions::*`
    /// use fntools::unstable::chain::FnExtChain;
    ///
    /// let add_two = |a: i32| a + 2;
    /// let add_three = |a: i32| a + 3;
    /// let add_eight = add_two
    ///     .chain(add_three)
    ///     .chain(add_three);
    ///
    /// assert_eq!(add_eight(4), 12);
    /// ```
    ///
    /// For more info see [`chain`]
    ///
    /// [`chain`]: crate::unstable::chain::chain
    fn supply(self, argument: T) -> Supply<T, Self, A>;
}

impl<T, F, A> FnExtSupply<T, A> for F
where
    F: FnOnce<A>,
    A: TupleTake<Take = T>,
{
    fn supply(self, argument: T) -> Supply<T, Self, A> {
        Supply::new(argument, self)
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
