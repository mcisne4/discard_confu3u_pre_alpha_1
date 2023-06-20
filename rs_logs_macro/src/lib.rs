#[proc_macro_derive(Logger, attributes(config, info_msg, warn_msg, error_msg))]
pub fn logger_derive_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    item
}
