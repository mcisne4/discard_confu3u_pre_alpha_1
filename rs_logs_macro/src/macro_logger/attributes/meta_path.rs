use super::ParsedAttr;

pub fn parse_meta_path<'a>(
    meta: &'a syn::Path,
    attr: &'a syn::Attribute,
) -> syn::Result<Option<ParsedAttr<'a>>> {
    let ident = &meta.segments[0].ident.to_string();

    let err_msg = match ident.as_str() {
        "info_msg" => Some(concat!(
            "The 'info_msg' attribute requires a string argument to be passed\n\nExample:\n",
            r#"#[info_msg = "my informational message"]"#
        )),
        "warn_msg" => Some(concat!(
            "The 'warn_msg' attribute requires a string argument to be passed\n\nExample:\n",
            r#"#[warn_msg = "my warning message"]"#
        )),
        "error_msg" => Some(concat!(
            "The 'error_msg' attribute requires a string argument to be passed\n\nExample:\n",
            r#"#[error_msg = "my error message"]"#
        )),
        "crate_idx" => Some(concat!(
            "The 'crate_idx' attribute requires an integer argument to be passed\n\nExample:\n",
            r#"#[crate_idx = 8]"#
        )),
        "mod_idx" => Some(concat!(
            "The 'mod_idx' attribute requires an integer argument to be passed\n\nExample:\n",
            r#"#[mod_idx = 172]"#
        )),
        "location" => Some(concat!(
            "The 'location' attribute requires a string argument to be passed\n\nExample:\n",
            r#"#[location = "rs_logs::example::module"]"#
        )),
        _ => None,
    };

    match err_msg {
        Some(message) => Err(syn::Error::new_spanned(attr, message)),
        None => Ok(None),
    }
}
