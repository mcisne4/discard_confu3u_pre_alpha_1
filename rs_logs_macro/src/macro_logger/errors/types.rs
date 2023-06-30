pub enum Attrs {
    CrateIdx,
    ModIdx,
    Location,
    InfoMsg,
    WarnMsg,
    ErrorMsg,
}
impl Attrs {
    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            Self::CrateIdx => EnumAttrs::CrateIdx.as_str(),
            Self::ModIdx => EnumAttrs::ModIdx.as_str(),
            Self::Location => EnumAttrs::Location.as_str(),
            Self::InfoMsg => VariantAttrs::InfoMsg.as_str(),
            Self::WarnMsg => VariantAttrs::WarnMsg.as_str(),
            Self::ErrorMsg => VariantAttrs::ErrorMsg.as_str(),
        }
    }
}

pub enum EnumAttrs {
    CrateIdx,
    ModIdx,
    Location,
}
impl EnumAttrs {
    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            Self::CrateIdx => "crate_idx",
            Self::ModIdx => "mod_idx",
            Self::Location => "location",
        }
    }
}

pub enum VariantAttrs {
    InfoMsg,
    WarnMsg,
    ErrorMsg,
}
impl VariantAttrs {
    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            Self::InfoMsg => "info_msg",
            Self::WarnMsg => "warn_msg",
            Self::ErrorMsg => "error_msg",
        }
    }
}
