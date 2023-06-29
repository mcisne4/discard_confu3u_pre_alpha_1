pub type LogResult<T> = std::result::Result<T, LoggerError>;

pub struct LoggerError {
    error: venial::Error,
}
impl LoggerError {
    pub fn new<Tokens, Message>(tokens: Tokens, message: Message) -> Self
    where
        Tokens: quote::ToTokens,
        Message: std::fmt::Display,
    {
        Self {
            error: venial::Error::new_at_tokens(tokens, message),
        }
    }

    pub fn to_venial_error(self) -> venial::Error {
        self.error
    }

    pub fn to_token_stream(self) -> proc_macro::TokenStream {
        self.error.to_compile_error().into()
    }
}
