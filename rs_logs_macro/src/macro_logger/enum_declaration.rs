use crate::macro_logger::errors::{EnumErrors, ToVenialError};
use crate::macro_logger::LoggerResult;

pub fn declaration_is_enum<'a>(
    declaration: &'a venial::Declaration,
) -> LoggerResult<&'a venial::Enum> {
    match declaration.as_enum() {
        Some(yes) => Ok(yes),
        None => Err(EnumErrors::InvalidDeclarationType.at(declaration)),
    }
}
