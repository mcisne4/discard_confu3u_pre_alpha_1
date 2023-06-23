use super::ParsedAttr;

pub fn parse_meta_metavalue<'a>(
    meta: &'a syn::MetaNameValue,
    attr: &'a syn::Attribute,
) -> syn::Result<Option<ParsedAttr<'a>>> {
    //

    Ok(None)
}
