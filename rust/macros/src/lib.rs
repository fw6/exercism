#[macro_export]
macro_rules! hashmap {
    (@count) => (0usize);
    (@count $x:tt $($xs:tt)*) => (1usize + $crate::hashmap!(@count $($xs)*));

    ($($key:expr => $value:expr,)+) => { $crate::hashmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = $crate::hashmap!(@count $($key),*);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            $(
                _map.insert($key, $value);
            )*
            _map
        }
    };
}

// https://dietcode.io/t/tt-munchers/
// https://stackoverflow.com/questions/34304593/counting-length-of-repetition-in-macro
