pub trait Sealed {}

impl Sealed for () {}

impl<S: Sealed> Sealed for &'_ S {}

impl<S: Sealed> Sealed for &'_ mut S {}

macro_rules! tuple_impl {
    ($( $types:ident, )*) => {
        impl<$( $types, )*> Sealed for ($( $types, )*)
        where
            last_type!($( $types, )*): ?Sized,
        {}
    };
}

for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, # tuple_impl);
