mod enum_declaration;
mod errors;

pub use errors::{logger_errors, LoggerResult};

pub fn macro_logger(item: proc_macro::TokenStream) -> LoggerResult<proc_macro2::TokenStream> {
    let item = proc_macro2::TokenStream::from(item);

    // --- Parse TokenStream --- //
    let declaration = venial::parse_declaration(item)?;

    // --- Validate Enum Type --- //
    let _enum_ast = enum_declaration::declaration_is_enum(&declaration)?;

    // --- To TokenStream --- //
    Ok(quote::quote! {
      //
    })
}
