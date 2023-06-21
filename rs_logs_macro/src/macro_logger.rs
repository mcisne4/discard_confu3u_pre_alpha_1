mod enum_data;

pub fn macro_logger(item: proc_macro::TokenStream) -> deluxe::Result<proc_macro2::TokenStream> {
    let item2: proc_macro2::TokenStream = item.into();

    // --- Parse TokenStream --- //
    let mut ast = syn::parse2::<syn::DeriveInput>(item2)?;

    // --- Enum Data --- //
    let _enum_data = enum_data::get_enum_data(&mut ast)?;

    // --- To TokenStream --- //
    Ok(quote::quote! {
      //
    })
}
