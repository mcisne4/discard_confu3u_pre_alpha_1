mod logger_derive;
use logger_derive::logger_derive;

#[proc_macro_derive(Logger, attributes(config, info_msg, warn_msg, error_msg))]
pub fn logger_derive_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match logger_derive(item) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
