#[macro_export]
macro_rules! method_impl {
    ($fn_name:ident, $method_variant:expr) => {
        pub fn $fn_name<F>(&mut self, path: &'static str, handler: F)
        where
            F: Fn(Request, Response) + Send + Sync + 'static,
        {
            self.register($method_variant, path, handler);
        }
    };
}
