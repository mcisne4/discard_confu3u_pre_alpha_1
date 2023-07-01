use super::example_code::{enum_example, ExampleCode};
use super::{Attrs, EnumAttrs, ToVenialError, VariantAttrs};
use venial::Error as VenialError;

pub enum ValueErrors {
    Empty(Attrs),
}
impl ValueErrors {
    fn at<Tokens: quote::ToTokens>(self, tokens: Tokens, is_eq_value: bool) -> venial::Error {
        match self {
            Self::Empty(attr) => value_is_empty(tokens, attr, is_eq_value),
        }
    }
}

fn value_is_empty<Tokens: quote::ToTokens>(
    tokens: Tokens,
    attr: Attrs,
    is_eq_value: bool,
) -> VenialError {
    let mut msg = format!(
        "No value provided for '{}' attribute. The '{}' attribute requires ",
        attr.as_str(),
        attr.as_str()
    );
    msg += match attr {
        Attrs::CrateIdx => "an integer value to be passed",
        Attrs::ModIdx => "an integer value to be passed",
        _ => "a string value to be passed",
    };

    msg += ExampleCode::LineExample.as_str();

    venial::Error::new_at_tokens(tokens, msg)
}
