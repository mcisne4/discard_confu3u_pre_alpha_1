pub fn format_variant_msg(
    variant: &mut syn::Variant,
    log_msg: String,
) -> deluxe::Result<proc_macro2::TokenStream> {
    let match_line = match &variant.fields {
        syn::Fields::Named(_) => format_struct_field(variant, log_msg)?,
        syn::Fields::Unnamed(_) => format_touple_field(variant, log_msg)?,
        syn::Fields::Unit => format_no_fields(variant, log_msg)?,
    };

    Ok(match_line)
}

fn format_no_fields(
    variant: &mut syn::Variant,
    log_msg: String,
) -> deluxe::Result<proc_macro2::TokenStream> {
    let ident = &variant.ident;

    Ok(quote::quote! {
      #ident => write!(_msg, #log_msg),
    })
}

fn format_touple_field(
    variant: &mut syn::Variant,
    log_msg: String,
) -> deluxe::Result<proc_macro2::TokenStream> {
    let ident = &variant.ident;

    let field = variant.fields.len();

    todo!()
}

fn format_struct_field(
    variant: &mut syn::Variant,
    log_msg: String,
) -> deluxe::Result<proc_macro2::TokenStream> {
    todo!()
}
