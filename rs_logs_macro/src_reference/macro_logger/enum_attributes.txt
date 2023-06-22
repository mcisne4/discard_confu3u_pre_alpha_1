// #[derive(deluxe::ExtractAttributes)]
// #[deluxe(attributes(config))]
// struct LogConfig<'a> {
//     crate_idx: u8,
//     mod_idx: u8,
//     location: String,
// }

use super::parse_attribute;

struct LogConfig<'a> {
    crate_idx: u8,
    mod_idx: u8,
    location: &'a str,
}

pub fn get_enum_attributes<'a>(ast: &'a mut syn::DeriveInput) -> syn::Result<()> {
    //     if ast.attrs.len() == 0 {
    //         return Err(syn::Error::new_spanned(
    //             &ast,
    //             "Implementation of Logger requires a 'config' attribute for the enum

    // Example:

    // #[derive(Logger)]
    // #[config(
    //     crate_idx = 1,
    //     mod_idx = 2,
    //     location = \"module::location::path\"
    // )]
    // enum MyLogger {
    //     ...
    // }
    // ",
    //         ));
    //     }

    for attr in &ast.attrs {
        // eprintln!("{:#?}", attr);
        // let attr_parsed_opt = parse_attribute(attr)?;

        if let Some(attr_parsed) = parse_attribute::parse_attribute(attr)? {
            match attr_parsed.ident.to_string().as_str() {
                "info_msg" => {
                    return Err(syn::Error::new_spanned(
                        attr,
                        "The '#[info_msg(...)]' attribute can only be used for enum variants",
                    ))
                }
                "warn_msg" => {
                    return Err(syn::Error::new_spanned(
                        attr,
                        "The '#[warn_msg(...)]' attribute can only be used for enum variants",
                    ));
                }
                "error_msg" => {
                    return Err(syn::Error::new_spanned(
                        attr,
                        "The '#[error_msg(...)]' attribute can only be used for enum variants",
                    ));
                }
                _ => (),
            }
        }
    }

    // println!("{:#?}", ast);

    Ok(())
}
