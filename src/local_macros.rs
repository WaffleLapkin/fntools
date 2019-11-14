// TODO: clean macros/remove them

/// Call macro for tuples
///
/// `tuple_impls!(C, B, A, ; cb)` will call `cb` with
/// - `cb!(A,)`
/// - `cb!(A, B)`
/// - `cb!(A, B, C)`
macro_rules! for_tuples {
    ( $( $types:ident, )* @ # $cb:ident) => {
        $cb!($( $types, )*);
    };
    ( $( $types:ident, )* @ $ty:ident, $( $rest:ident, )* # $cb:ident) => {
        $cb!($( $types, )*);
        for_tuples!($( $types, )* $ty, @ $( $rest, )* # $cb);
    };
    ( $ty:ident, $( $rest:ident, )* # $cb:ident) => {
        for_tuples!( $ty, @ $( $rest, )* # $cb);
    };
    () => {};
}

macro_rules! for_tuples_tt {
    ( $( $types:ident [$e:tt], )* @ # $cb:ident) => {};
    ( $( $types:ident [$e:tt], )* @ $ty:ident [$e2:tt], $( $rest:ident [$e3:tt], )* # $cb:ident) => {
        $cb!($( $types [$e], )* $ty [$e2],);
        for_tuples_tt!($( $types [$e], )* $ty [$e2], @ $( $rest [$e3], )* # $cb);
    };
    ( $ty:ident [$e:tt], $( $rest:ident [$e2:tt], )* # $cb:ident) => {
        $cb!( $ty [$e],);
        for_tuples_tt!( $ty [$e], @ $( $rest [$e2], )* # $cb);
    };
    () => {};
}

macro_rules! for_tuples_tt_last {
    ( $( $types:ident [$e:tt], )* @ # $cb:ident) => {};
    ( $( $types:ident [$e:tt], )* @ $ty:ident [$e2:tt], $( $rest:ident [$e3:tt], )* # $cb:ident) => {
        $cb!($( $types [$e], )* @ $ty [$e2],);
        for_tuples_tt_last!($( $types [$e], )* $ty [$e2], @ $( $rest [$e3], )* # $cb);
    };
    ( $ty:ident [$e:tt], $( $rest:ident [$e2:tt], )* # $cb:ident) => {
        $cb!( @ $ty [$e],);
        for_tuples_tt_last!( $ty [$e], @ $( $rest [$e2], )* # $cb);
    };
    () => {};
}
