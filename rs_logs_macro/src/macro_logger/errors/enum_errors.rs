use super::example_code::{enum_example, ExampleCode};
use super::{EnumAttrs, ToVenialError, VariantAttrs};
use venial::Error as VenialError;

pub enum EnumErrors {
    InvalidDeclarationType,

    HasVariantAttr(VariantAttrs),

    MissingAttr(EnumAttrs),
    MultipleAttrs(EnumAttrs),
}
impl ToVenialError for EnumErrors {
    fn at<Tokens: quote::ToTokens>(self, tokens: Tokens) -> venial::Error {
        match self {
            // --- Declaration is not an Enum --- //
            Self::InvalidDeclarationType => VenialError::new_at_tokens(
                tokens,
                "Implementation of 'Logger' is only available for enum types",
            ),

            // --- Variant Attribute in Enum Declaration --- //
            Self::HasVariantAttr(attr) => VenialError::new_at_tokens(
                tokens,
                format!(
                    "The '{}' attribute can only be used on enum variants",
                    attr.as_str()
                ),
            ),

            // --- Missing Attribute in Enum Declaration --- //
            Self::MissingAttr(attr) => VenialError::new_at_tokens(
                tokens,
                format!(
                    "The '{}' attribute is required for enum declarations{}",
                    attr.as_str(),
                    enum_example()
                ),
            ),

            // --- Multiple Attributes in Enum Declaration --- //
            Self::MultipleAttrs(attr) => VenialError::new_at_tokens(
                tokens,
                format!(
                    "The '{}' attribute is only allowed once per declaration",
                    attr.as_str()
                ),
            ),
        }
    }
}
