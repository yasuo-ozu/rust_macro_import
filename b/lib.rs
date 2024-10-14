pub use b_macro;

#[macro_export]
macro_rules! do_macro_b {
    () => {
        $crate::b_macro::do_macro_b!($crate)
    };
}

macro_rules! emit_items {
    () => {
        b_macro::emit_items!($crate);
    };
}

emit_items!();

macro_rules! emit_fn_call {
    () => {
        b_macro::emit_fn_call!($crate)
    };
}

pub fn do_fn_b_2() -> &'static str {
    emit_fn_call!()
}

pub fn do_fn_b() -> &'static str {
    "hello from b"
}
