mod number_attr;
mod string_attr;

use super::ParsedAttr;

pub fn parse_meta_metavalue<'a>(
    meta: &'a syn::MetaNameValue,
    attr: &'a syn::Attribute,
) -> syn::Result<Option<ParsedAttr<'a>>> {
    let ident = &meta.path.segments[0].ident;

    let parsed_attr = match ident.to_string().as_str() {
        "info_msg" => Some(string_attr::parse_string_attr(ident, meta, attr)?),
        "warn_msg" => Some(string_attr::parse_string_attr(ident, meta, attr)?),
        "error_msg" => Some(string_attr::parse_string_attr(ident, meta, attr)?),
        "crate_idx" => Some(number_attr::parse_number_attr(ident, meta, attr, true)?),
        "mod_idx" => Some(number_attr::parse_number_attr(ident, meta, attr, false)?),
        "location" => Some(string_attr::parse_string_attr(ident, meta, attr)?),
        _ => None,
    };

    Ok(parsed_attr)
}
