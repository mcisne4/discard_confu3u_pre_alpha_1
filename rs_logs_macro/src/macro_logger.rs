pub fn macro_logger(item: proc_macro::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    // --- Parse TokenStream --- //
    let _ast = syn::parse::<syn::DeriveInput>(item)?;

    // --- To TokenStream --- //
    Ok(quote::quote! {
      //
    })
}
