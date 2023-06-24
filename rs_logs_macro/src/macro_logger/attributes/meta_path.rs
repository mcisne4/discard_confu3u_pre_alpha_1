use super::attr_errors::AttrError;
use super::ParsedAttr;

pub fn parse_meta_path<'a>(
    meta: &'a syn::Path,
    attr: &'a syn::Attribute,
) -> syn::Result<Option<ParsedAttr<'a>>> {
    let ident = &meta.segments[0].ident.to_string();

    let attr_error = match ident.as_str() {
        "info_msg" => Some(AttrError::EmptyInfoMsg),
        "warn_msg" => Some(AttrError::EmptyWarnMsg),
        "error_msg" => Some(AttrError::EmptyErrorMsg),
        "crate_idx" => Some(AttrError::EmptyCrateIdx),
        "mod_idx" => Some(AttrError::EmptyModIdx),
        "location" => Some(AttrError::EmptyLocation),
        _ => None,
    };

    match attr_error {
        Some(err) => Err(syn::Error::new_spanned(attr, err.to_string())),
        None => Ok(None),
    }
}
