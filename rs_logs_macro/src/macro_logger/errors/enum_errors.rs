use super::{Attrs, EnumAttrs, ToVenialError, VariantAttrs};
use venial::Error as VenialError;

pub enum EnumErrors {
    InvalidDeclarationType,

    HasVariantAttr(VariantAttrs),
}
impl ToVenialError for EnumErrors {
    fn at<Tokens: quote::ToTokens>(self, tokens: Tokens) -> venial::Error {
        match self {
            Self::InvalidDeclarationType => VenialError::new_at_tokens(
                tokens,
                "Implementation of 'Logger' is only available for enum types",
            ),
            Self::HasVariantAttr(attr) => VenialError::new_at_tokens(
                tokens,
                format!(
                    "The '{}' attribute can only be used on enum variants",
                    attr.as_str()
                ),
            ),
        }
    }
}
