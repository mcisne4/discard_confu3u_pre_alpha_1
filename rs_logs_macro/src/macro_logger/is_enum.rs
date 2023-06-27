use super::errors::LoggerErrors;
use quote::ToTokens;

pub fn declaration_is_enum<'a>(
    declaration: &'a venial::Declaration,
) -> Result<&'a venial::Enum, venial::Error> {
    match declaration.as_enum() {
        Some(yes) => Ok(yes),
        None => Err(LoggerErrors::NotAnEnum(declaration.to_token_stream()).as_error()),
    }
}
