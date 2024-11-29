#[macro_export]
macro_rules! loader {
    ($name: expr) => {
        $crate::ModLoader {
            id: $name.to_lowercase(),
            name: $name.into(),
        }
    };
}

#[macro_export]
macro_rules! loaders {
    [$($name: expr),*$(,)?] => {
        vec![$($crate::loader!($name)),*]
    };
}
