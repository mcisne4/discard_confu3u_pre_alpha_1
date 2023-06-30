use super::{Attrs, EnumAttrs, ToVenialError, VariantAttrs};
use venial::Error as VenialError;

pub enum VariantErrors {
    HasEnumAttr(EnumAttrs),
}
impl ToVenialError for VariantErrors {
    fn at<Tokens: quote::ToTokens>(self, tokens: Tokens) -> venial::Error {
        match self {
            Self::HasEnumAttr(attr) => VenialError::new_at_tokens(
                tokens,
                format!(
                    "The '{}' attribute can only be used on enum declarations",
                    attr.as_str()
                ),
            ),
        }
    }
}
