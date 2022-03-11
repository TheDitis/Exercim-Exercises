#[macro_export(local_inner_macros)]
macro_rules! hashmap {
    ($($key:expr => $val:expr),*) => {
        {
            use ::std::collections::HashMap;
            let mut map = HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
    ($($key:expr => $val:expr,)+) => {
        hashmap!($($key => $val),+)
    };
    (@single $($x:tt)*) => (());
    () => {
        { HashMap::new() }
    };
}