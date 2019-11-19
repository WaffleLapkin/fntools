use crate::sealed::Sealed;

/// Trait for conversation refs to tuples into tuples of refs. e.g. `&(A, B)` into `(&A, &B)`
pub trait AsRefTuple<'l>: Sealed {
    type Target;

    // This actually should look like this:
    // `fn as_ref<'a>(&'a self) -> Self::Target<'a>`
    // But we don't have GAT yet :(
    fn as_ref(&'l self) -> Self::Target;
}

/// Trait for conversation unique refs to tuples into tuples of unique refs. e.g. `&mut (A, B)` into
/// `(&mut A, &mut B)`
pub trait AsMutTuple<'l>: Sealed {
    type Target;

    // This actually should look like this:
    // `fn as_ref<'a>(&'a mut self) -> Self::Target<'a>`
    // But we don't have GAT yet :(
    fn as_mut(&'l mut self) -> Self::Target;
}

impl<'l> AsRefTuple<'l> for () {
    type Target = ();

    fn as_ref(&'l self) -> Self::Target {}
}

impl<'l> AsMutTuple<'l> for () {
    type Target = ();

    fn as_mut(&'l mut self) -> Self::Target {}
}

macro_rules! tuple_impl {
    ($( $types:ident, )*) => {
        impl<'l, $( $types: 'l, )*> AsRefTuple<'l> for ($( $types, )*)
        where
            last_type!($( $types, )*): ?Sized,
        {
            type Target = ($( &'l $types, )*);

            #[inline]
            #[allow(non_snake_case)]
            fn as_ref(&'l self) -> Self::Target {
                let ($( $types, )*) = self;
                ($( $types, )*)
            }
        }

        impl<'l, $( $types: 'l, )*> AsMutTuple<'l> for ($( $types, )*)
        where
            last_type!($( $types, )*): ?Sized,
        {
            type Target = ($( &'l mut $types, )*);

            #[inline]
            #[allow(non_snake_case)]
            fn as_mut(&'l mut self) -> Self::Target {
                let ($( $types, )*) = self;
                ($( $types, )*)
            }
        }
    };
}

for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, # tuple_impl);
