pub struct ParsedAttr<'a> {
    pub ident: &'a syn::Ident,
    pub value: String,
    pub attr: &'a syn::Attribute,
}
