mod enum_errors;
mod types;
mod variant_errors;

pub type LoggerResult<T> = std::result::Result<T, venial::Error>;

pub trait ToVenialError {
    fn at<Tokens: quote::ToTokens>(self, tokens: Tokens) -> venial::Error;
}

pub use enum_errors::EnumErrors;
pub use types::{Attrs, EnumAttrs, VariantAttrs};
pub use variant_errors::VariantErrors;
