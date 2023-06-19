mod enum_attributes;
mod enum_data;
mod impl_statement;

pub fn logger_derive(item: proc_macro::TokenStream) -> deluxe::Result<proc_macro2::TokenStream> {
    let item: proc_macro2::TokenStream = item.into();

    // --- Parse TokenStream --- //
    let mut ast = syn::parse2::<syn::DeriveInput>(item)?;

    // --- Enum Data --- //
    let mut enum_data = enum_data::get_enum_data(&mut ast)?;

    // --- Enum Attributes --- //
    let (log_prefix, log_location) = enum_attributes::get_enum_attributes(&mut ast)?;
    let log_prefix = log_prefix.as_str();
    let log_location = log_location.as_str();

    // --- Impl Statement --- //
    let impl_stmt = impl_statement::get_impl_statement(&ast);

    // --- To TokenStream --- //
    Ok(quote::quote! {
        #impl_stmt {
          fn log_path() -> (&'static str, &'static str) {
            (#log_prefix, #log_location)
          }
        }
    })
}
