#[derive(deluxe::ExtractAttributes, Default, Debug)]
#[deluxe(attributes(info_msg))]
#[deluxe(default)]
struct InfoMsg(String);

#[derive(deluxe::ExtractAttributes, Default, Debug)]
#[deluxe(attributes(warn_msg))]
#[deluxe(default)]
struct WarnMsg(String);

#[derive(deluxe::ExtractAttributes, Default, Debug)]
#[deluxe(attributes(error_msg))]
#[deluxe(default)]
struct ErrorMsg(String);

pub enum LogType {
    INFO,
    WARN,
    ERROR,
}
impl LogType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::INFO => "INFO",
            Self::WARN => "WARN",
            Self::ERROR => "ERROR",
        }
    }
}

pub fn get_variant_attribute(variant: &mut syn::Variant) -> deluxe::Result<(LogType, String)> {
    let msg_info = deluxe::extract_attributes::<syn::Variant, InfoMsg>(variant)?;
    let msg_warn = deluxe::extract_attributes::<syn::Variant, WarnMsg>(variant)?;
    let msg_error = deluxe::extract_attributes::<syn::Variant, ErrorMsg>(variant)?;

    let info_msg = msg_info.0;
    let warn_msg = msg_warn.0;
    let error_msg = msg_error.0;

    let info_has_data = info_msg.len() > 0;
    let warn_has_data = warn_msg.len() > 0;
    let err_has_data = error_msg.len() > 0;

    let msg_no_attrs = "Could not find an appropiate log attribute for the variant\nAdd an 'info_msg', 'warn_msg', or 'error_msg' attribute to the variant";
    let msg_multiple_attrs = "Multiple log attributes found\nA variant should only have either one 'info_msg', one 'warn_msg', or one 'error_msg' attribute";

    let (log_type, log_msg) = match info_has_data {
        true => match warn_has_data {
            true => return Err(syn::Error::new_spanned(&variant, msg_multiple_attrs)),
            false => match err_has_data {
                true => return Err(syn::Error::new_spanned(&variant, msg_multiple_attrs)),
                false => (LogType::INFO, info_msg),
            },
        },
        false => match warn_has_data {
            true => match err_has_data {
                true => return Err(syn::Error::new_spanned(&variant, msg_multiple_attrs)),
                false => (LogType::WARN, warn_msg),
            },
            false => match err_has_data {
                true => (LogType::ERROR, error_msg),
                false => return Err(syn::Error::new_spanned(&variant, msg_no_attrs)),
            },
        },
    };

    Ok((log_type, log_msg))
}
