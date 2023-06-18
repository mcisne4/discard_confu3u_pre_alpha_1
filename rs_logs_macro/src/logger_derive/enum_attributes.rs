use syn::DeriveInput;

#[derive(deluxe::ExtractAttributes, Default)]
#[deluxe(attributes(config))]
#[deluxe(default)]
struct EnumConfig {
    crate_idx: u8,
    mod_idx: u8,
    location: String,
}

pub fn get_enum_attributes(ast: &mut DeriveInput) -> deluxe::Result<(String, String)> {
    let config = deluxe::extract_attributes::<DeriveInput, EnumConfig>(ast)?;

    let log_prefix = format!("{:X}{:02X}", config.crate_idx, config.mod_idx);
    let log_location = config.location;

    Ok((log_prefix, log_location))
}
