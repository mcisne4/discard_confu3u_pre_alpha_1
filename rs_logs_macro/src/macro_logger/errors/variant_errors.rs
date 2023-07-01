use super::example_code::variant_example;
use super::{EnumAttrs, ToVenialError};
use venial::Error as VenialError;

pub enum VariantErrors {
    HasEnumAttr(EnumAttrs),

    MissingAttr,
    MultipleAttrs,
}
impl ToVenialError for VariantErrors {
    fn at<Tokens: quote::ToTokens>(self, tokens: Tokens) -> venial::Error {
        match self {
            // --- Enum Attribute used for Variant --- //
            Self::HasEnumAttr(attr) => VenialError::new_at_tokens(
                tokens,
                format!(
                    "The '{}' attribute can only be used on enum declarations",
                    attr.as_str()
                ),
            ),

            // --- Missing Attribute --- //
            Self::MissingAttr => VenialError::new_at_tokens(
                tokens,
                format!(
                    "{}{}{}",
                    "The 'Logger' implementation requires each enum variant to have ",
                    "either an 'info_msg', a 'warn_msg', or a 'error_msg' attribute",
                    variant_example()
                ),
            ),

            // --- Multiple Attributes --- //
            Self::MultipleAttrs => VenialError::new_at_tokens(
                tokens,
                concat!(
                    "Multiple attributes were provided for the variant. ",
                    "Only one 'info_msg' attribute, 'warn_msg' attribute, or ",
                    "'error_msg' attribute is allowed"
                ),
            ),
        }
    }
}
