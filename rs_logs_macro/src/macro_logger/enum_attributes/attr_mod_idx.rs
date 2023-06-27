use crate::macro_logger::{
    attr_data::{parse_integer_data, ParsedInt},
    errors::ModIdxErrors,
};

pub fn parse_mod_idx(attr: &venial::Attribute) -> Result<String, venial::Error> {
    match &attr.value {
        venial::AttributeValue::Empty => return Err(ModIdxErrors::ValueIsEmpty(attr).as_error()),
        venial::AttributeValue::Equals(_punc, tokens) => {
            let value_token = &tokens[0];

            match parse_integer_data(value_token, false) {
                ParsedInt::Unparseable => Err(ModIdxErrors::Unparseable(attr).as_error()),
                ParsedInt::InvalidNumber => Err(ModIdxErrors::InvalidNumber(attr).as_error()),
                ParsedInt::Value(value) => {
                    eprintln!("RESULT: {}", value);
                    Ok(value)
                }
                ParsedInt::InvalidType => Err(ModIdxErrors::InvalidType(attr).as_error()),
            }
        }
        venial::AttributeValue::Group(_group_span, tokens) => {
            let value_token = match tokens.len() {
                0 => return Err(ModIdxErrors::ValueIsEmpty(attr).as_error()),
                1 => &tokens[0],
                _ => return Err(ModIdxErrors::MultipleValues(attr).as_error()),
            };

            match parse_integer_data(value_token, true) {
                ParsedInt::Unparseable => Err(ModIdxErrors::Unparseable(attr).as_error()),
                ParsedInt::InvalidNumber => Err(ModIdxErrors::InvalidNumber(attr).as_error()),
                ParsedInt::Value(value) => {
                    eprintln!("RESULT: {}", value);
                    Ok(value)
                }
                ParsedInt::InvalidType => Err(ModIdxErrors::InvalidType(attr).as_error()),
            }
        }
    }
}
