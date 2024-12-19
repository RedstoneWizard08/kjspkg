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
macro_rules! tag {
    ($id: expr, $name: expr, $icon: expr) => {
        $crate::Tag {
            id: $id.into(),
            name: $name.into(),
            icon: $icon.into(),
        }
    };
}

#[macro_export]
macro_rules! loaders {
    [$($name: expr),*$(,)?] => {
        vec![$($crate::loader!($name)),*]
    };
}

#[macro_export]
macro_rules! tags {
    [$($id: expr, $name: expr, $icon: expr);*$(;)?] => {
        vec![$($crate::tag!($id, $name, $icon)),*]
    };
}
