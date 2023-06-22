mod types;

pub fn parse_attribute<'a>(attr: &'a syn::Attribute) -> syn::Result<()> {
    match &attr.meta {
        syn::Meta::List(meta) => eprintln!("List Meta: {:?}", "meta"),
        syn::Meta::NameValue(meta) => eprintln!("NameValue Meta: {:?}", "meta"),
        syn::Meta::Path(meta) => eprintln!("Path Meta: {:?}", "meta"),
    }

    Ok(())
}
