use super::ParsedAttr;

pub fn parse_meta_list<'a>(
    meta: &'a syn::MetaList,
    attr: &'a syn::Attribute,
) -> syn::Result<Option<ParsedAttr<'a>>> {
    //

    Ok(None)
}
