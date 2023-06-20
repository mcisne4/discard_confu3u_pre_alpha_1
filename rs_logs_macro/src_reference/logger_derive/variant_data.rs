// use std::fmt::Write;
use super::variant_attributes;
use super::variant_msg;

pub fn get_variant_data(
    enum_data: &mut syn::DataEnum,
) -> deluxe::Result<Vec<(syn::Ident, &'static str)>> {
    let mut data: Vec<(syn::Ident, &'static str)> = Vec::new();

    for variant in enum_data.variants.iter_mut() {
        let ident = variant.ident.to_owned();
        let (log_type, log_msg) = variant_attributes::get_variant_attribute(variant)?;

        let msg = variant_msg::format_variant_msg(variant, log_msg)?;
    }

    // for variant in enum_data.variants.iter_mut() {
    //     let ident = variant.ident.to_owned();
    //     eprintln!("\n{:#?}:", ident.to_string());
    //     // eprintln!("{:#?}", variant);

    //     let mut x = String::new();
    //     write!(x, "Hello {} {}", "World", 45)
    //         .map_err(|e| syn::Error::new_spanned(&variant.ident, e.to_string()))?;
    //     // let vl: Result<String, std::fmt::Error> = write!(x, "Hello {}", "world");
    //     // let x = write!(std::fmt, "{}", ident);

    //     eprintln!("{}", x);

    //     match variant.fields.len() {
    //         0 => eprintln!("No field data"),
    //         _ => eprintln!("Has {} fields", variant.fields.len()),
    //     }

    //     match &variant.fields {
    //         syn::Fields::Named(n) => eprintln!("Named"),
    //         syn::Fields::Unnamed(n) => eprintln!("Unnamed"),
    //         syn::Fields::Unit => (),
    //     }
    //     // if variant.fields.len() > 0 {
    //     // }

    //     for field in variant.fields.iter_mut() {
    //         // eprintln!("\n{:#?}", field);

    //         // match field.ty {
    //         //     //
    //         // }

    //         // match &field.ty {
    //         //   syn::Type::
    //         //     _ => eprintln!("Type could not be found"),
    //         // }

    //         // let z = syn::parse2::<syn::Type>(x)?;

    //         // let field_type = field.ty;

    //         // eprintln!("{}: {:#?}||{:#?}", ident, x, y);
    //     }

    //     let msg_info = deluxe::extract_attributes::<Variant, InfoMsg>(variant)?;
    //     let msg_warn = deluxe::extract_attributes::<Variant, WarnMsg>(variant)?;
    //     let msg_error = deluxe::extract_attributes::<Variant, ErrorMsg>(variant)?;

    //     // eprintln!("{}: {:#?}", ident, msg_info);
    //     // eprintln!("{}: {:#?}", ident, variant);

    //     // let fields = variant.discriminant;
    // }

    Ok(data)
}
