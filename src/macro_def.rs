/// Chains many functions.
///
/// ```
/// use fntools::chain_many;
///
/// let to_16 = |i: i8| i16::from(i);
/// let to_32 = |i: i16| i32::from(i);
/// let to_64 = |i: i32| i64::from(i);
///
/// // execution order: to_16 -> to_32 -> to_64
/// let i8_to_i64 = chain_many!(to_16, to_32, to_64);
///
/// assert_eq!(i8_to_i64(8i8), 8i64);
/// ```
///
/// ## Note
///
/// Without `nightly` feature this macro will use `fntools::chain`, and with
/// `nightly` feature this macro will use `fntools::unstable::chain::chain`.
#[macro_export]
#[cfg(feature = "nightly")]
macro_rules! chain_many {
    ($head:expr, $tail:expr) => {
        $crate::unstable::chain($head, $tail)
    };

    ($head:expr, $( $tail:expr ),+ $(,)?) => {
        $crate::unstable::chain($head, $crate::chain_many!( $( $tail ),+ ))
    };
}

/// Chains many functions.
///
/// ```
/// use fntools::chain_many;
///
/// let to_16 = |i: i8| i16::from(i);
/// let to_32 = |i: i16| i32::from(i);
/// let to_64 = |i: i32| i64::from(i);
///
/// // execution order: to_16 -> to_32 -> to_64
/// let i8_to_i64 = chain_many!(to_16, to_32, to_64);
///
/// assert_eq!(i8_to_i64(8i8), 8i64);
/// ```
///
/// ## Note
///
/// Without `nightly` feature this macro will use `fntools::chain`, and with
/// `nightly` feature this macro will use `fntools::unstable::chain::chain`.
#[macro_export]
#[cfg(not(feature = "nightly"))]
macro_rules! chain_many {
    ($head:expr, $tail:expr) => {
        $crate::chain($head, $tail)
    };

    ($head:expr, $( $tail:expr ),+ $(,)?) => {
        $crate::chain($head, $crate::chain_many!( $( $tail ),+ ))
    };
}

/// Same as [`compose_many`](crate::compose_many), but this macro uses
/// `fntools::chain_once`
#[macro_export]
macro_rules! chain_many_once {
    ($head:expr, $tail:expr) => {
        $crate::chain_once($head, $tail)
    };

    ($head:expr, $( $tail:expr ),+ $(,)?) => {
        $crate::chain_once($head, $crate::chain_many_once!( $( $tail ),+ ))
    };
}

/// Same as [`compose_many`](crate::compose_many), but this macro uses
/// `fntools::chain_mut`
#[macro_export]
macro_rules! chain_many_mut {
    ($head:expr, $tail:expr) => {
        $crate::chain_mut($head, $tail)
    };

    ($head:expr, $( $tail:expr ),+ $(,)?) => {
        $crate::chain_mut($head, $crate::chain_many_mut!( $( $tail ),+ ))
    };
}

/// Composes many functions.
///
/// ```
/// use fntools::compose_many;
///
/// let to_16 = |i: i8| i16::from(i);
/// let to_32 = |i: i16| i32::from(i);
/// let to_64 = |i: i32| i64::from(i);
///
/// // execution order: to_16 -> to_32 -> to_64
/// let i8_to_i64 = compose_many!(to_64, to_32, to_16);
///
/// assert_eq!(i8_to_i64(8i8), 8i64);
/// ```
///
/// ## Note
///
/// Without `nightly` feature this macro will use `fntools::compose`, and with
/// `nightly` feature this macro will use `fntools::unstable::compose::compose`.
#[macro_export]
#[cfg(feature = "nightly")]
macro_rules! compose_many {
    ($head:expr, $tail:expr) => {
        $crate::unstable::compose($head, $tail)
    };

    ($head:expr, $( $tail:expr ),+ $(,)?) => {
        $crate::unstable::compose($head, $crate::compose_many!( $( $tail ),+ ))
    };
}

/// Composes many functions.
///
/// ```
/// use fntools::compose_many;
///
/// let to_16 = |i: i8| i16::from(i);
/// let to_32 = |i: i16| i32::from(i);
/// let to_64 = |i: i32| i64::from(i);
///
/// // execution order: to_16 -> to_32 -> to_64
/// let i8_to_i64 = compose_many!(to_64, to_32, to_16);
///
/// assert_eq!(i8_to_i64(8i8), 8i64);
/// ```
///
/// ## Note
///
/// Without `nightly` feature this macro will use `fntools::compose`, and with
/// `nightly` feature this macro will use `fntools::unstable::compose::compose`.
#[macro_export]
#[cfg(not(feature = "nightly"))]
macro_rules! compose_many {
    ($head:expr, $tail:expr) => {
        $crate::compose($head, $tail)
    };

    ($head:expr, $( $tail:expr ),+ $(,)?) => {
        $crate::compose($head, $crate::compose_many!( $( $tail ),+ ))
    };
}

/// Same as [`compose_many`](crate::compose_many), but this macro uses
/// `fntools::compose_once`
#[macro_export]
macro_rules! compose_many_once {
    ($head:expr, $tail:expr) => {
        $crate::compose_once($head, $tail)
    };

    ($head:expr, $( $tail:expr ),+ $(,)?) => {
        $crate::compose_once($head, $crate::compose_many_once!( $( $tail ),+ ))
    };
}

/// Same as [`compose_many`](crate::compose_many), but this macro uses
/// `fntools::compose_mut`
#[macro_export]
macro_rules! compose_many_mut {
    ($head:expr, $tail:expr) => {
        $crate::compose_mut($head, $tail)
    };

    ($head:expr, $( $tail:expr ),+ $(,)?) => {
        $crate::compose_mut($head, $crate::compose_many_mut!( $( $tail ),+ ))
    };
}
