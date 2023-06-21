pub fn get_enum_data<'a>(ast: &'a mut syn::DeriveInput) -> deluxe::Result<&'a syn::DataEnum> {
    let enum_data = match &ast.data {
        syn::Data::Enum(data) => data,
        syn::Data::Struct(data) => return Err(syn::Error::new_spanned(
            data.struct_token,
            "Implementation of Logger for structs is not supported\nConsider using an enum instead",
        )),
        syn::Data::Union(data) => return Err(syn::Error::new_spanned(
            data.union_token,
            "Implementation of Logger for unions is not supported\nConsider using an enum instead",
        )),
    };

    Ok(enum_data)
}
