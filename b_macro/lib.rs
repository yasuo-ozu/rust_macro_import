use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use template_quote::quote;

#[proc_macro]
pub fn emit_items(krate: TokenStream1) -> TokenStream1 {
    let krate: TokenStream = krate.into();
    quote! {
        pub fn emitted_fn_b() -> &'static str {
            #krate::do_fn_b()
        }

        #[macro_export]
        macro_rules! emitted_macro_b {
            () => { #krate::do_fn_b() };
        }
    }
    .into()
}

#[proc_macro]
pub fn emit_fn_call(krate: TokenStream1) -> TokenStream1 {
    let krate: TokenStream = krate.into();
    quote! {#krate::do_fn_b()}.into()
}

#[proc_macro]
pub fn do_macro_b(krate: TokenStream1) -> TokenStream1 {
    let krate: TokenStream = krate.into();
    quote! (#krate::do_fn_b()).into()
}
