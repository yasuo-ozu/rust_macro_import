#[macro_export]
macro_rules! do_macro_a {
    () => {
        $crate::do_fn_a()
    };
}

#[macro_export]
macro_rules! do_macro_b {
    () => {
        $crate::imp::do_fn_b()
    };
}

#[macro_export]
macro_rules! do_macro_macro_b {
    () => {
        $crate::imp::do_macro_b!()
    };
}

#[macro_export]
macro_rules! do_macro_macro_b2 {
    () => {
        $crate::imp::emitted_macro_b!()
    };
}

#[macro_export]
macro_rules! do_macro_macro_b3 {
    () => {
        $crate::imp::do_fn_b_2()
    };
}

pub mod imp {
    pub use b::do_fn_b;
    pub use b::do_fn_b_2;
    pub use b::do_macro_b;
    pub use b::emitted_macro_b;
}

pub fn do_fn_a() -> &'static str {
    "hello from a"
}
