use quote::ToTokens;
use venial::Error as VenialError;

pub enum EnumDeclaration {
    NotAnEnum,
}
impl EnumDeclaration {
    pub fn at<Tokens: ToTokens>(self, tokens: Tokens) -> VenialError {
        match self {
            Self::NotAnEnum => VenialError::new_at_tokens(
                tokens,
                "Implementation of 'Logger' is only available for enum types",
            ),
        }
    }
}
