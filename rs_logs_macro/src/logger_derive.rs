mod impl_statement;

pub fn logger_derive(item: proc_macro::TokenStream) -> deluxe::Result<proc_macro2::TokenStream> {
    let item: proc_macro2::TokenStream = item.into();

    // --- Parse TokenStream --- //
    let ast = syn::parse2::<syn::DeriveInput>(item)?;

    // --- Impl Statement --- //
    let impl_stmt = impl_statement::get_impl_statement(&ast);

    // --- To TokenStream --- //
    Ok(quote::quote! {
        #impl_stmt {
        }
    })
}
