mod attributes;
mod enum_data;

pub fn macro_logger(item: proc_macro::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    // --- Parse TokenStream --- //
    let ast = syn::parse::<syn::DeriveInput>(item)?;

    // --- Enum Data --- //
    let _data = enum_data::get_enum_data(&ast)?;

    ////// TESTING //////
    /////////////////////
    testing(&ast)?;
    /////////////////////
    /////////////////////

    // --- To TokenStream --- //
    Ok(quote::quote! {
      //
    })
}

fn testing<'a>(ast: &'a syn::DeriveInput) -> syn::Result<()> {
    for attr in &ast.attrs {
        match attributes::parse_attribute(attr) {
            Ok(result) => eprintln!("\nRESULTS: {:#?}", result),
            Err(e) => eprintln!("{:?}", e.to_string()),
        }
    }

    Ok(())
}
