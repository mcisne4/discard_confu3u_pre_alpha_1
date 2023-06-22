pub struct ParsedAttr<'a> {
    pub ident: &'a syn::Ident,
    pub args: ParsedAttrArg,
    pub attr: &'a syn::Attribute,
}

#[derive(Debug)]
pub enum ParsedAttrArg {
    CrateIdx(u8),
    ModIdx(u8),
    Location(String),
    InfoMsg(String),
    WarnMsg(String),
    ErrorMsg(String),
}
impl ParsedAttrArg {
    fn value(self) -> String {
        match self {
            Self::CrateIdx(val) => format!("{:1X}", val),
            Self::ModIdx(val) => format!("{:02X}", val),
            Self::Location(val) => val,
            Self::InfoMsg(val) => val,
            Self::WarnMsg(val) => val,
            Self::ErrorMsg(val) => val,
        }
    }
}
