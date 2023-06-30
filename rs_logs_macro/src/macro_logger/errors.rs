mod enum_declaration;

pub type LoggerResult<T> = std::result::Result<T, venial::Error>;

pub mod logger_errors {
    pub use super::enum_declaration::EnumDeclaration;
}
