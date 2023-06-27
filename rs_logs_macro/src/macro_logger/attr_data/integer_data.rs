use litrs::Literal;
use std::fmt::Write;

#[derive(Debug)]
pub enum ParsedInt {
    Value(String),
    Unparseable,
    InvalidNumber,
    InvalidType,
}

pub fn parse_integer_data(value_token: &proc_macro2::TokenTree, is_u4: bool) -> ParsedInt {
    match Literal::try_from(value_token) {
        Err(_) => ParsedInt::Unparseable,
        Ok(Literal::Integer(int_lit)) => {
            let x = parse_integer_literal(int_lit, is_u4);
            eprintln!("<> {:?}", x);
            x
        }
        Ok(_lit) => ParsedInt::InvalidType,
    }
}

fn parse_integer_literal(int_lit: litrs::IntegerLit<String>, is_u4: bool) -> ParsedInt {
    let str_value = int_lit.raw_input();

    let num = match str_value.parse::<usize>() {
        Err(_) => return ParsedInt::Unparseable,
        Ok(val) => val,
    };

    let value = match is_u4 {
        true => {
            if num > 15 {
                eprintln!("{} > 15", num);
                return ParsedInt::InvalidNumber;
            }

            let mut val = String::new();
            match write!(val, "{:01X}", num) {
                Ok(_) => val,
                Err(_) => String::from("X"),
            }
        }

        false => {
            if num > 255 {
                eprintln!("{} > 255", num);
                return ParsedInt::InvalidNumber;
            }

            let mut val = String::new();
            match write!(val, "{:02X}", num) {
                Ok(_) => val,
                Err(_) => String::from("XX"),
            }
        }
    };

    ParsedInt::Value(value)
}
