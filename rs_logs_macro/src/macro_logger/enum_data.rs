pub fn get_enum_data<'a>(ast: &'a syn::DeriveInput) -> syn::Result<&'a syn::DataEnum> {
    let data = match &ast.data {
        syn::Data::Enum(data) => data,
        syn::Data::Struct(data) => return Err(syn::Error::new_spanned(
          data.struct_token,
        "Implementation of Logger for structs is not supported\n\nConsider using an enum instead\n")),
        syn::Data::Union(data)=> return Err(syn::Error::new_spanned(
          data.union_token,
        "Implementation of Logger for unions is not supported\n\nConsider using an enum instead\n"))
    };

    Ok(data)
}
