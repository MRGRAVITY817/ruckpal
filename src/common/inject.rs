#[macro_export]
macro_rules! inject {
    ($name: ident) => {
        Box<dyn $name>
    };
}
