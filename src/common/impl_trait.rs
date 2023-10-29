#[macro_export]
macro_rules! impl_t {
    ($name: ident) => {
        Box<dyn $name>
    };
}
