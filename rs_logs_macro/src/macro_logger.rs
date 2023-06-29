mod errors;
mod is_enum;

pub use errors::{LogResult, LoggerError};

pub fn macro_logger(
    item: proc_macro::TokenStream,
) -> Result<proc_macro2::TokenStream, venial::Error> {
    let item = proc_macro2::TokenStream::from(item);

    // --- Parse TokenStream --- //
    let declaration = venial::parse_declaration(item)?;

    // --- Validate Enum Type --- //
    let _enum_ast = is_enum::declaration_is_enum(&declaration)?;

    // --- To TokenStream --- //
    Ok(quote::quote! {
      //
    })
}
