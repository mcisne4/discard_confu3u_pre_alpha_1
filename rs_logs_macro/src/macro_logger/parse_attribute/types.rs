pub struct ParsedAttribute<'a> {
    pub ident: &'a syn::Ident,
    pub args: Vec<ParsedAttributeArg<'a>>,
}

pub struct ParsedAttributeArg<'a> {
    pub ident: &'a syn::Ident,
}

pub enum AttributeArg<'a> {
    Str(&'a str),
    Int(&'a usize),
}
