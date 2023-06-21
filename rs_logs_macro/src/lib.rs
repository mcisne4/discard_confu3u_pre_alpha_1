mod macro_logger;

#[proc_macro_derive(Logger, attributes(config, info_msg, warn_msg, error_msg))]
pub fn logger_derive_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match macro_logger::macro_logger(item) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
