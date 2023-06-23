mod meta_list;
mod meta_namevalue;
mod meta_path;
mod types;

pub use types::{ParsedAttr, ParsedAttrArg};

pub fn parse_attribute<'a>(attr: &'a syn::Attribute) -> syn::Result<Option<ParsedAttr>> {
    let parsed_attr = match &attr.meta {
        syn::Meta::List(meta) => meta_list::parse_meta_list(meta, attr)?,
        syn::Meta::NameValue(meta) => meta_namevalue::parse_meta_metavalue(meta, attr)?,
        syn::Meta::Path(meta) => meta_path::parse_meta_path(meta, attr)?,
    };

    Ok(parsed_attr)
}
