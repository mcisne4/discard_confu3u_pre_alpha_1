pub enum AttrError {
    EmptyInfoMsg,
    EmptyWarnMsg,
    EmptyErrorMsg,
    EmptyCrateIdx,
    EmptyModIdx,
    EmptyLocation,
    ValueInvalidTypeInfoMsg,
    ValueInvalidTypeWarnMsg,
    ValueInvalidTypeErrorMsg,
    ValueInvalidTypeCrateIdx,
    ValueInvalidTypeModIdx,
    ValueInvalidTypeLocation,
    InvalidRangeCrateIdx,
    InvalidRangeModIdx,
}
impl AttrError {
    pub fn to_string(&self) -> String {
        let err_msg = match self {
            Self::EmptyInfoMsg => concat!(
                "The 'info_msg' attribute requires a string argument to be passed\n\nExample:\n",
                r#"#[info_msg = "my informational message"]"#
            ),
            Self::EmptyWarnMsg => concat!(
                "The 'warn_msg' attribute requires a string argument to be passed\n\nExample:\n",
                r#"#[warn_msg = "my warning message"]"#
            ),
            Self::EmptyErrorMsg => concat!(
                "The 'error_msg' attribute requires a string argument to be passed\n\nExample:\n",
                r#"#[error_msg = "my error message"]"#
            ),
            Self::EmptyCrateIdx => concat!(
                "The 'crate_idx' attribute requires an integer argument to be passed\n\nExample:\n",
                r#"#[crate_idx = 8]"#
            ),
            Self::EmptyModIdx => concat!(
                "The 'mod_idx' attribute requires an integer argument to be passed\n\nExample:\n",
                r#"#[mod_idx = 172]"#
            ),
            Self::EmptyLocation => concat!(
                "The 'location' attribute requires a string argument to be passed\n\nExample:\n",
                r#"#[location = "rs_logs::example::module"]"#
            ),
            Self::ValueInvalidTypeInfoMsg => concat!(
                "The 'info_msg' attribute requires a string argument to be passed\n\nExample:\n",
                r#"#[info_msg = "my informational message"]"#
            ),
            Self::ValueInvalidTypeWarnMsg => concat!(
                "The 'warn_msg' attribute requires a string argument to be passed\n\nExample:\n",
                r#"#[warn_msg = "my warning message"]"#
            ),
            Self::ValueInvalidTypeErrorMsg => concat!(
                "The 'error_msg' attribute requires a string argument to be passed\n\nExample:\n",
                r#"#[error_msg = "my error message"]"#
            ),
            Self::ValueInvalidTypeCrateIdx => concat!(
                "The 'crate_idx' attribute requires an integer argument to be passed\n\nExample:\n",
                r#"#[crate_idx = 8]"#
            ),
            Self::ValueInvalidTypeModIdx => concat!(
                "The 'mod_idx' attribute requires an integer argument to be passed\n\nExample:\n",
                r#"#[mod_idx = 172]"#
            ),
            Self::ValueInvalidTypeLocation => concat!(
                "The 'location' attribute requires a string argument to be passed\n\nExample:\n",
                r#"#[location = "rs_logs::example::module"]"#
            ),
            Self::InvalidRangeCrateIdx => {
                "The 'crate_idx' value should be an integer between 0 and 15"
            }
            Self::InvalidRangeModIdx => {
                "The 'mod_idx' value should be an integer between 0 and 255"
            }
        };

        err_msg.to_owned()
    }
}
