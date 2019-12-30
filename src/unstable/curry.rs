use crate::tuple::{push::TuplePush, take::TupleTake};
use std::marker::PhantomData;

/// **Extremely bad** curring.
///
/// ## Examples
/// ```
/// use fntools::unstable::curry::curry;
/// use std::ops::Add;
///
/// let fun = curry(i32::add);
/// let res = fun(2)(2)();
/// //                 ^^ ---- yep, you need this :(
/// assert_eq!(res, 4);
/// ```
#[inline]
pub fn curry<A, F>(f: F) -> Curry<(), F, A, A>
where
    F: FnOnce<A>,
{
    Curry::new(f)
}

pub struct Curry<T, F, Args, RemArgs> {
    supplied: T,
    f: F,
    marker: PhantomData<dyn Fn(Args, RemArgs)>,
}

impl<F, A> Curry<(), F, A, A> {
    #[inline]
    pub fn new(f: F) -> Self
    where
        F: FnOnce<A>,
    {
        Curry {
            supplied: (),
            f,
            marker: PhantomData,
        }
    }
}

impl<T, F, A, RA> Curry<T, F, A, RA> {
    #[inline]
    pub fn into_inner(self) -> (T, F) {
        let Curry { supplied, f, .. } = self;
        (supplied, f)
    }

    #[inline]
    pub fn as_inner(&self) -> (&T, &F) {
        let Curry { supplied, f, .. } = self;
        (supplied, f)
    }
}

impl<D, Rem, FArgs, F> FnOnce<(Rem::Take,)> for Curry<D, F, FArgs, Rem>
where
    F: FnOnce<FArgs>,
    Rem: TupleTake,
    D: TuplePush<Rem::Take>,
{
    type Output = Curry<D::Res, F, FArgs, Rem::Rem>;

    #[inline]
    extern "rust-call" fn call_once(self, (arg,): (Rem::Take,)) -> Self::Output {
        let Curry { supplied, f, .. } = self;
        Curry {
            supplied: supplied.push(arg),
            f,
            marker: PhantomData,
        }
    }
}

impl<FArgs, F> FnOnce<()> for Curry<FArgs, F, FArgs, ()>
where
    F: FnOnce<FArgs>,
{
    type Output = F::Output;

    #[inline]
    extern "rust-call" fn call_once(self, _: ()) -> Self::Output {
        let Curry { supplied, f, .. } = self;
        f.call_once(supplied)
    }
}

impl<FArgs, F> FnMut<()> for Curry<FArgs, F, FArgs, ()>
where
    F: FnMut<FArgs>,
    FArgs: Clone,
{
    #[inline]
    extern "rust-call" fn call_mut(&mut self, _: ()) -> Self::Output {
        let Curry { supplied, f, .. } = self;
        f.call_mut(supplied.clone())
    }
}

impl<FArgs, F> Fn<()> for Curry<FArgs, F, FArgs, ()>
where
    F: Fn<FArgs>,
    FArgs: Clone,
{
    #[inline]
    extern "rust-call" fn call(&self, _: ()) -> Self::Output {
        let Curry { supplied, f, .. } = self;
        f.call(supplied.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::unstable::curry::Curry;

    #[test]
    fn one_fn() {
        let fun = |a| a * 2;
        let fun = Curry::new(fun)(4);

        assert_eq!(fun(), 8);
        assert_eq!(fun(), 8);
    }

    #[test]
    fn one_fn_once() {
        struct UnCopy;
        let uncopy = UnCopy;

        let fun = move |a| {
            let _ = uncopy;
            a * 2
        };
        let fun = Curry::new(fun)(4);

        assert_eq!(fun(), 8);
    }

    #[test]
    fn one_fn_mut() {
        let mut var = false;

        let fun = |a| {
            var = true;
            a * 2
        };
        let mut fun = Curry::new(fun)(4);

        // TODO: for some reason type inheritance doesn't work
        assert_eq!(fun.call_mut(()), 8);
    }

    #[test]
    fn many_fn() {
        let fun = |a: i32, b: String, c: &str, d: i8| format!("{}{}{}{}", a, b, c, d);
        let fun = Curry::new(fun);
        let fun = fun(12)(String::from("O_o"))("hell(o)")(4);

        assert_eq!(fun(), "12O_ohell(o)4");
        assert_eq!(fun(), "12O_ohell(o)4");
    }
}
