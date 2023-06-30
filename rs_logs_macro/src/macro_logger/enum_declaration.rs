use crate::macro_logger::logger_errors::EnumDeclaration;
use crate::macro_logger::LoggerResult;

pub fn declaration_is_enum<'a>(
    declaration: &'a venial::Declaration,
) -> LoggerResult<&'a venial::Enum> {
    match declaration.as_enum() {
        Some(yes) => Ok(yes),
        None => Err(EnumDeclaration::NotAnEnum.at(declaration)),
    }
}
