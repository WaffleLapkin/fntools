/// Trait that converts any type[^1] to tuple.
///
/// This trait converts
/// - tuples of arity `0..12`[^2] to themselves.
/// - `T` to `(T,)`
///
/// [^1]: any `Sized` type
///
/// [^2]: Rust current type system can't express "tuple of any size" (see
/// [Draft RFC: variadic generics] for proposes how to fix this) so this lib
/// follows the [stdlib] in implementing traits on tuples of arity 12 or less.
///
/// [Draft RFC: variadic generics]: https://github.com/rust-lang/rfcs/issues/376
/// [stdlib]: https://doc.rust-lang.org/std/primitive.tuple.html#trait-implementations
pub trait AutoTuple<T> {
    fn auto_tuple(self) -> T;
}

impl<T> AutoTuple<(T,)> for T {
    #[inline]
    fn auto_tuple(self) -> (T,) {
        (self,)
    }
}

impl AutoTuple<()> for () {
    #[inline]
    fn auto_tuple(self) {}
}

macro_rules! tuple_impl {
    ( $( $types:ident, )* ) => {
        impl<$( $types ),*> AutoTuple<Self> for ( $( $types, )* )
        {
            #[inline]
            fn auto_tuple(self) -> Self {
                self
            }
        }
    };
}

/// Implement `AutoTuple<T> for T` where `T` is tuple type.
///
/// `tuple_impls!(C, B, A,)` will generate impls for
/// - (A,)
/// - (A, B)
/// - (A, B, C)
macro_rules! tuple_impls {
    ( $( $types:ident, )* @ ) => {
        tuple_impl!($( $types, )*);
    };
    ( $( $types:ident, )* @ $ty:ident, $( $rest:ident, )* ) => {
        tuple_impl!($( $types, )*);
        tuple_impls!($( $types, )* $ty, @ $( $rest, )*);
    };
    ( $ty:ident, $( $rest:ident, )* ) => {
        tuple_impls!( $ty, @ $( $rest, )* );
    };
    () => {};
}

// Generate impl for tuples of arity 1..12
tuple_impls!(A, B, C, D, E, F, G, H, I, J, K, L,);

#[cfg(test)]
mod tests {
    use crate::tuple::auto_tuple::AutoTuple;

    /// Converts `val` to `U` using [`AutoTuple`] and compares it with `rhs`, if
    /// they are equal, return `true`, otherwise `false`.
    ///
    /// [`AutoTuple`]: crate::tuple::auto_tuple::AutoTuple
    fn helper<T, U>(val: T, rhs: U) -> bool
    where
        T: AutoTuple<U>,
        U: PartialEq<U>,
    {
        val.auto_tuple() == rhs
    }

    #[test]
    fn to_arity1_tuple() {
        assert!(helper(1, (1,)));
        assert!(helper((1,), ((1,),)));
        assert!(helper((1, 0), ((1, 0),)));
    }

    #[test]
    fn to_self() {
        fn inner<T>(val: T) -> bool
        where
            T: PartialEq<T> + AutoTuple<T> + Clone,
        {
            helper(val.clone(), val)
        }

        assert!(inner(()));

        assert!(inner((1,)));
        assert!(inner((1, 2)));
        assert!(inner((1, 2, 3)));
        assert!(inner((1, 2, 3, 4)));

        assert!(inner(((),)));
        assert!(inner(((), ())));
    }
}
