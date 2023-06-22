mod enum_data;

pub fn macro_logger(item: proc_macro::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    // --- Parse TokenStream --- //
    let ast = syn::parse::<syn::DeriveInput>(item)?;

    // --- Enum Data --- //
    let _data = enum_data::get_enum_data(&ast)?;

    // --- To TokenStream --- //
    Ok(quote::quote! {
      //
    })
}
