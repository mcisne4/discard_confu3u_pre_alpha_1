mod attr_crate_idx;
mod attr_location;
mod attr_mod_idx;
mod enum_def_tokens;

use crate::macro_logger::errors::{CrateIdxErrors, LoggerErrors, ModIdxErrors};

pub fn parse_enum_attributes<'a>(enum_ast: &'a venial::Enum) -> Result<(), venial::Error> {
    if enum_ast.attributes.is_empty() {
        let tokens = enum_def_tokens::get_enum_def_tokens(&enum_ast);
        return Err(LoggerErrors::EnumMissingAttrs(tokens).as_error());
    }

    let mut val_crate_idx: Option<String> = None;
    let mut val_mod_idx: Option<String> = None;
    let mut val_location: Option<String> = None;

    for attr in &enum_ast.attributes {
        let attr_name = attr.path[0].to_string();

        match attr_name.as_str() {
            "info_msg" => return Err(LoggerErrors::EnumHasInfoMsg(attr).as_error()),
            "warn_msg" => return Err(LoggerErrors::EnumHasWarnMsg(attr).as_error()),
            "error_msg" => return Err(LoggerErrors::EnumHasErrorMsg(attr).as_error()),
            "crate_idx" => match val_crate_idx {
                None => {
                    let value = attr_crate_idx::parse_crate_idx(&attr)?;
                    val_crate_idx = Some(value);
                }
                Some(_) => return Err(CrateIdxErrors::MultipleAttributes(&attr).as_error()),
            },
            "mod_idx" => match val_mod_idx {
                None => {
                    let value = attr_mod_idx::parse_mod_idx(&attr)?;
                    val_mod_idx = Some(value);
                }
                Some(_) => return Err(ModIdxErrors::MultipleAttributes(attr).as_error()),
            },
            // "location" => match val_location {
            //     Some(_) => return Err(LoggerErrors::EnumMultipleLocation(attr).as_error()),
            //     None => {
            //         let value = attr_location::parse_location(&attr)?;
            //         val_location = Some(value);
            //     }
            // },
            _ => (),
        }
    }

    Ok(())
}
