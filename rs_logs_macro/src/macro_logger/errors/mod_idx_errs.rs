use venial::Error;

pub enum ModIdxErrors<T: quote::ToTokens> {
    /// Multiple 'mod_idx' attributes found
    MultipleAttributes(T),

    /// The attribute has no value
    ValueIsEmpty(T),

    /// The value cannot be parsed
    Unparseable(T),

    /// The value is not an integer
    InvalidType(T),

    /// The integer value is not between 0 and 255
    InvalidNumber(T),

    /// Multiple arguments passed as the value
    MultipleValues(T),
}

impl<T: quote::ToTokens> ModIdxErrors<T> {
    pub fn as_error(&self) -> venial::Error {
        match self {
            // --- Multiple Attributes --- //
            Self::MultipleAttributes(tokens) => {
                Error::new_at_tokens(tokens, "The 'mod_idx' attribute must only be used once")
            }

            // --- Value is Empty --- //
            Self::ValueIsEmpty(tokens) => Error::new_at_tokens(
                tokens,
                concat!(
                    "No value found for the 'mod_idx' attribute. ",
                    "The 'mod_idx' attributes requires an integer value to be passed\n\n",
                    "Examples:\n- #[mod_idx = 206]\n- #[mod_idx(206)]\n"
                ),
            ),

            // --- Value Cannot be Parsed --- //
            Self::Unparseable(tokens) => Error::new_at_tokens(
                tokens,
                concat!(
                    "Could not parse the 'mod_idx' attribute\n\n",
                    "The attribute should be formatted in either of the following formats:\n",
                    "  - #[mod_idx = int]\n",
                    "  - #[mod_idx(int)]\n",
                    "Where 'int' is an integer between 0 and 255\n"
                ),
            ),

            // --- Value is not an Integer --- //
            Self::InvalidType(tokens) => Error::new_at_tokens(
                tokens,
                concat!(
                    "An invalid data type was passed for the 'mod_idx' attribute value. ",
                    "The value should be an integer between 0 and 255\n"
                ),
            ),

            // --- Value is not a u4 --- //
            Self::InvalidNumber(tokens) => Error::new_at_tokens(
                tokens,
                "The 'mod_idx' attribute value should be an integer between 0 and 255\n",
            ),

            // --- Tuple Value = Multiple Arguments --- //
            Self::MultipleValues(tokens) => Error::new_at_tokens(
                tokens,
                concat!(
                    "Multiple arguments were passed as the value for the 'mod_idx' attribute. ",
                    "The value should be a single integer between 0 and 255\n"
                ),
            ),
        }
    }
}
