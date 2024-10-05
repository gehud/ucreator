#[macro_export]
macro_rules! __utils__for_each_tuple___ {
    ( $m:ident !! ) => (
        $m! { }
    );
    ( $m:ident !! $h:ident, $($t:ident,)* ) => (
        $m! { $h, $($t),* }
        $crate::__utils__for_each_tuple___! { $m !! $($t,)* }
    );
}

#[deprecated]
#[macro_export]
macro_rules! __utils__for_each_tuple_16__ {
    ( $m:ident ) => (
        $crate::__utils__for_each_tuple___! { $m !! A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, }
    );
}
