mod list_meta;
mod namevalue_meta;
mod path_meta;
mod types;

use types::ParsedAttribute;

pub fn parse_attribute<'a>(attr: &'a syn::Attribute) -> syn::Result<Option<ParsedAttribute<'a>>> {
    let parsed_attr: Option<ParsedAttribute<'a>> = match &attr.meta {
        syn::Meta::List(meta) => list_meta::parse_list_meta(meta, attr)?,
        syn::Meta::NameValue(meta) => namevalue_meta::parse_namevalue_meta(meta, attr)?,
        syn::Meta::Path(meta) => path_meta::parse_path_meta(meta, attr)?,
    };

    Ok(parsed_attr)
}
