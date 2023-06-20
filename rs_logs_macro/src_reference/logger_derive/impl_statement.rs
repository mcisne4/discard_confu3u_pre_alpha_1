pub fn get_impl_statement(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let ident = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    quote::quote! {
      impl #impl_generics Logger for #ident #type_generics #where_clause
    }
}
