use super::super::attr_errors::AttrError;
use super::ParsedAttr;

pub fn parse_number_attr<'a>(
    ident: &'a syn::Ident,
    meta: &'a syn::MetaNameValue,
    attr: &'a syn::Attribute,
    single_hex_char: bool,
) -> syn::Result<ParsedAttr<'a>> {
    let has_num_value = match &meta.value {
        syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
            syn::Lit::Int(lit_int) => match lit_int.base10_parse::<usize>() {
                Ok(value) => Some(value),
                Err(_) => None,
            },
            _ => None,
        },
        _ => None,
    };

    match has_num_value {
        None => {
            let attr_err = match ident.to_string().as_str() {
                "crate_idx" => AttrError::ValueInvalidTypeCrateIdx,
                "mod_idx" => AttrError::ValueInvalidTypeModIdx,
                _ => unreachable!(),
            };

            Err(syn::Error::new_spanned(attr, attr_err.to_string()))
        }
        Some(num) => {
            let value = match single_hex_char {
                true => {
                    if num > 15 {
                        return Err(syn::Error::new_spanned(
                            &meta.value,
                            AttrError::InvalidRangeCrateIdx.to_string(),
                        ));
                    }
                    format!("{:01X}", num)
                }
                false => {
                    if num > 255 {
                        return Err(syn::Error::new_spanned(
                            &meta.value,
                            AttrError::InvalidRangeModIdx.to_string(),
                        ));
                    }
                    format!("{:02X}", num)
                }
            };

            Ok(ParsedAttr { ident, value, attr })
        }
    }
}
