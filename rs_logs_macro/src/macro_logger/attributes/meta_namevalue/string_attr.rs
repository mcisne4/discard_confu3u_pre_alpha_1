use super::super::attr_errors::AttrError;
use super::ParsedAttr;

pub fn parse_string_attr<'a>(
    ident: &'a syn::Ident,
    meta: &'a syn::MetaNameValue,
    attr: &'a syn::Attribute,
) -> syn::Result<ParsedAttr<'a>> {
    let has_str_value = match &meta.value {
        syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
            syn::Lit::Str(lit_str) => Some(lit_str.value()),
            _ => None,
        },
        _ => None,
    };

    match has_str_value {
        Some(value) => Ok(ParsedAttr { ident, value, attr }),
        None => {
            let attr_err = match ident.to_string().as_str() {
                "info_msg" => AttrError::ValueInvalidTypeInfoMsg,
                "warn_msg" => AttrError::ValueInvalidTypeWarnMsg,
                "error_msg" => AttrError::ValueInvalidTypeErrorMsg,
                "location" => AttrError::ValueInvalidTypeLocation,
                _ => unreachable!(),
            };

            Err(syn::Error::new_spanned(attr, attr_err.to_string()))
        }
    }
}
