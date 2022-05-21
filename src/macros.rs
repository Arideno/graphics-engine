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

#[macro_export]
macro_rules! m {
    () => {
        {
            Matrix::new(0, 0, vec![])
        }
    };
    ($( $( $x: expr ),*);*) => {
        {
            let data_as_nested_array = [ $( [ $($x),* ] ),* ];
            let rows = data_as_nested_array.len();
            let cols = data_as_nested_array[0].len();
            let data_as_flat_array: Vec<f32> = data_as_nested_array.into_iter()
                .flat_map(|row| row.into_iter())
                .collect();
            Matrix::new(rows, cols, data_as_flat_array)
        }
    }
}