use std::marker::PhantomData;

use crate::tuple::{at_least_2::AtLeast2, push::TuplePush, take::TupleTake};

/// Curring.
///
/// ## Examples
/// ```
/// use fntools::unstable::curry;
/// use std::ops::Add;
///
/// let fun = curry(i32::add);
/// let res = fun(2)(2);
/// assert_eq!(res, 4);
/// ```
#[inline]
pub fn curry<F, Rem>(f: F) -> Curry<(), F, Rem>
where
    F: FnOnce<Rem>,
{
    Curry::new(f)
}

pub struct Curry<Supplied, F, Remaining> {
    supplied: Supplied,
    f: F,
    marker: PhantomData<fn(Remaining)>,
}

// Nothing is supplied, everything is remaining
impl<F, Rem> Curry<(), F, Rem> {
    #[inline]
    pub fn new(f: F) -> Self
    where
        F: FnOnce<Rem>,
    {
        Curry {
            supplied: (),
            f,
            marker: PhantomData,
        }
    }
}

impl<S, F, Rem> Curry<S, F, Rem> {
    #[inline]
    pub fn into_inner(self) -> (S, F) {
        let Curry { supplied, f, .. } = self;
        (supplied, f)
    }

    #[inline]
    pub fn as_inner(&self) -> (&S, &F) {
        let Curry { supplied, f, .. } = self;
        (supplied, f)
    }
}

impl<S, F, Rem> FnOnce<(Rem::Take,)> for Curry<S, F, Rem>
where
    // This is needed to remove ambiguity with
    // later "type Output = F::Output;" impls
    //
    // The funniest thing about this bound is
    // that it can't be changed to
    // Rem: TupleTake,
    // <Rem as TupleTake>::Rem: TupleTake,
    // Nevertheless the fact that AtLeast2 is
    // implemented with blanked impl for all
    // those types
    Rem: AtLeast2,
    Rem::Rem: TupleTake,
    S: TuplePush<Rem::Take>,
{
    type Output = Curry<S::Res, F, Rem::Rem>;

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

impl<S, F, A> FnOnce<(A,)> for Curry<S, F, (A,)>
where
    S: TuplePush<A>,
    F: FnOnce<S::Res>,
{
    type Output = F::Output;

    #[inline]
    extern "rust-call" fn call_once(self, (arg,): (A,)) -> Self::Output {
        let Curry { supplied, f, .. } = self;
        let supplied = supplied.push(arg);
        f.call_once(supplied)
    }
}

impl<S, F, A> FnMut<(A,)> for Curry<S, F, (A,)>
where
    S: TuplePush<A>,
    F: FnMut<S::Res>,
    S: Clone,
{
    #[inline]
    extern "rust-call" fn call_mut(&mut self, (arg,): (A,)) -> Self::Output {
        let Curry { supplied, f, .. } = self;
        let supplied = supplied.clone().push(arg);
        f.call_mut(supplied)
    }
}

impl<S, F, A> Fn<(A,)> for Curry<S, F, (A,)>
where
    S: TuplePush<A>,
    F: Fn<S::Res>,
    S: Clone,
{
    #[inline]
    extern "rust-call" fn call(&self, (arg,): (A,)) -> Self::Output {
        let Curry { supplied, f, .. } = self;
        let supplied = supplied.clone().push(arg);
        f.call(supplied)
    }
}

#[cfg(test)]
mod tests {
    use crate::unstable::curry::Curry;

    #[test]
    fn one_fn() {
        let fun = |a| a * 2;
        let val = Curry::new(fun)(4);

        assert_eq!(val, 8);
    }

    #[test]
    fn one_fn_once() {
        struct UnCopy;
        let uncopy = UnCopy;

        let fun = move |a| {
            let _ = uncopy;
            a * 2
        };
        let val = Curry::new(fun)(4);

        assert_eq!(val, 8);
    }

    #[test]
    fn one_fn_mut() {
        let mut var = false;

        let fun = |a| {
            var = true;
            a * 2
        };
        // TODO: for some reason type inheritance doesn't work
        let val = Curry::new(fun).call_mut((4,));

        assert_eq!(val, 8);
    }

    #[test]
    fn many_fn() {
        let fun = |a: i32, b: String, c: &str, d: i8| format!("{}{}{}{}", a, b, c, d);
        let fun = Curry::new(fun);
        let val = fun(12)(String::from("O_o"))("hell(o)")(4);

        assert_eq!(val, "12O_ohell(o)4");
    }
}
