#[macro_export]
macro_rules! impl_froms {
    ($n:ident: $($x:ident),*) => {
        $(
            impl From<$x> for $n {
                fn from(x: $x) -> $n {
                    $n::$x(x)
                }
            }
        )*
    }
}
