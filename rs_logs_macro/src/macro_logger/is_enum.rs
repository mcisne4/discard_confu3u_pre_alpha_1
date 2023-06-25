use quote::ToTokens;

pub fn declaration_is_enum<'a>(
    declaration: &'a venial::Declaration,
) -> Result<&'a venial::Enum, venial::Error> {
    match declaration.as_enum() {
        Some(yes) => Ok(yes),
        None => Err(venial::Error::new_at_tokens(
            declaration.to_token_stream(),
            "Implementation of 'Logger' is only available for enum types",
        )),
    }
}
