use venial::Error;

pub enum LoggerErrors<T: quote::ToTokens> {
    /// Data type is not an enum
    NotAnEnum(T),

    /// Enum does not implement all the required attributes
    EnumMissingAttrs(T),

    /// 'info_msg' attribute should only be used on enum variants
    EnumHasInfoMsg(T),

    /// 'warn_msg' attribute should only be used on enum variants
    EnumHasWarnMsg(T),

    /// 'error_msg' attribute should only be used on enum variants
    EnumHasErrorMsg(T),
}

impl<T: quote::ToTokens> LoggerErrors<T> {
    pub fn as_error(&self) -> venial::Error {
        match self {
            // --- Incorrect Data Type --- //
            Self::NotAnEnum(tokens) => Error::new_at_tokens(
                tokens,
                "Implementation of 'Logger' is only available for enum types",
            ),

            // --- Enum is Missing Attributes --- //
            Self::EnumMissingAttrs(tokens) => Error::new_at_tokens(
                tokens,
                concat!(
                    "The 'Logger' implementation requires a 'crate_idx' attribute, ",
                    "a 'mod_idx' attribute, and a 'location' attribute to be ",
                    "included in the enum definition\n\nExample:\n",
                    "#[derive(Logger)]\n",
                    "#[crate_idx = 12]\n",
                    "#[mod_idx = 210]\n",
                    "#[location = \"rs_logs::module::path\"]\n",
                    "enum MyLogger { ... }\n"
                ),
            ),

            // --- Invalid Attributes: 'info_msg' --- //
            Self::EnumHasInfoMsg(tokens) => Error::new_at_tokens(
                tokens,
                "The 'info_msg' attribute can only be used for enum variants",
            ),
            // --- Invalid Attributes: 'warn_msg' --- //
            Self::EnumHasWarnMsg(tokens) => Error::new_at_tokens(
                tokens,
                "The 'warn_msg' attribute can only be used for enum variants",
            ),
            // --- Invalid Attributes: 'error_msg' --- //
            Self::EnumHasErrorMsg(tokens) => Error::new_at_tokens(
                tokens,
                "The 'error_msg' attribute can only be used for enum variants",
            ),
        }
    }
}
