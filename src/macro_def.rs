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
#[macro_export]
#[cfg(not(feature = "stable"))]
macro_rules! chain_many {
    ($head:expr, $tail:expr) => {
        $crate::unstable::chain::chain($head, $tail)
    };

    ($head:expr, $( $tail:expr ),+ $(,)?) => {
        $crate::unstable::chain::chain($head, $crate::chain_many!( $( $tail ),+ ))
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
#[macro_export]
#[cfg(feature = "stable")]
macro_rules! chain_many {
    ($head:expr, $tail:expr) => {
        $crate::chain($head, $tail)
    };

    ($head:expr, $( $tail:expr ),+ $(,)?) => {
        $crate::chain($head, $crate::chain_many!( $( $tail ),+ ))
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
#[macro_export]
#[cfg(not(feature = "stable"))]
macro_rules! compose_many {
    ($head:expr, $tail:expr) => {
        $crate::unstable::compose::compose($head, $tail)
    };

    ($head:expr, $( $tail:expr ),+ $(,)?) => {
        $crate::unstable::compose::compose($head, $crate::compose_many!( $( $tail ),+ ))
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
#[macro_export]
#[cfg(feature = "stable")]
macro_rules! compose_many {
    ($head:expr, $tail:expr) => {
        $crate::compose($head, $tail)
    };

    ($head:expr, $( $tail:expr ),+ $(,)?) => {
        $crate::compose($head, $crate::compose_many!( $( $tail ),+ ))
    };
}
