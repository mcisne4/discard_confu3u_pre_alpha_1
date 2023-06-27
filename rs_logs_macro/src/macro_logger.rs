mod attr_data;
mod enum_attributes;
mod errors;
mod is_enum;

pub fn macro_logger(
    item: proc_macro::TokenStream,
) -> Result<proc_macro2::TokenStream, venial::Error> {
    let item = proc_macro2::TokenStream::from(item);

    // --- Parse TokenStream --- //
    let declaration = venial::parse_declaration(item)?;

    // --- Validate Enum Type --- //
    let enum_ast = is_enum::declaration_is_enum(&declaration)?;

    // --- Enum Data --- //
    let enum_data = enum_attributes::parse_enum_attributes(enum_ast)?;

    // --- To TokenStream --- //
    Ok(quote::quote! {
      //
    })
}
