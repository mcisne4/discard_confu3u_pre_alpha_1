mod string_attr;
use super::ParsedAttr;

pub fn parse_meta_list<'a>(
    meta: &'a syn::MetaList,
    attr: &'a syn::Attribute,
) -> syn::Result<Option<ParsedAttr<'a>>> {
    let ident = &meta.path.segments[0].ident;

    let parsed_attr = match ident.to_string().as_str() {
        "info_msg" => Some(string_attr::parse_sting_attr(ident, meta, attr)?),
        _ => None,
    };

    Ok(None)
}
