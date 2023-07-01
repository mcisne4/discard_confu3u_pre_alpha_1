pub enum ExampleCode {
    LineExample,
    LineEnum,
    LineImpl,
    LineVariant1,
    LineVariant2,
    LineVariant3,

    EqCrateIdx,
    EqModIdx,
    EqLocation,
    EqInfoMsg,
    EqWarnMsg,
    EqErrorMsg,

    TplCrateIdx,
    TplModIdx,
    TplLocation,
    TplInfoMsg,
    TplWarnMsg,
    TplErrorMsg,
}
impl ExampleCode {
    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            ExampleCode::LineExample => "\n\nExample:\n",
            ExampleCode::LineEnum => "enum MyLogger {\n  ",
            ExampleCode::LineImpl => "#[derive(Logger)]\n",

            ExampleCode::LineVariant1 => "  Variant1,\n",
            ExampleCode::LineVariant2 => "  Variant2,\n",
            ExampleCode::LineVariant3 => "  Variant3,\n",

            ExampleCode::EqCrateIdx => "#[crate_idx = 15]\n",
            ExampleCode::EqModIdx => "#[mod_idx = 249]\n",
            ExampleCode::EqLocation => "#[location = \"rs_logs::example::path\"]\n",
            ExampleCode::EqInfoMsg => "#[info_msg = \"This is an info message\"]\n",
            ExampleCode::EqWarnMsg => "#[warn_msg = \"This is a warning message\"]\n",
            ExampleCode::EqErrorMsg => "#[error_msg = \"This is an error message\"]\n",

            ExampleCode::TplCrateIdx => "#[crate_idx(15)]\n",
            ExampleCode::TplModIdx => "#[mod_idx(249)]\n",
            ExampleCode::TplLocation => "#[location(\"rs_logs::example::path\")]\n",
            ExampleCode::TplInfoMsg => "#[info_msg(\"This is an info message\")]\n",
            ExampleCode::TplWarnMsg => "#[warn_msg(\"This is a warning message\")]\n",
            ExampleCode::TplErrorMsg => "#[error_msg(\"This is an error message\")]\n",
        }
    }
}

pub fn full_example() -> String {
    format!(
        "{}{}{}{}{}{}{}  {}{}  {}{}}}",
        ExampleCode::LineImpl.as_str(),
        ExampleCode::EqCrateIdx.as_str(),
        ExampleCode::EqModIdx.as_str(),
        ExampleCode::EqLocation.as_str(),
        ExampleCode::LineEnum.as_str(),
        ExampleCode::EqInfoMsg.as_str(),
        ExampleCode::LineVariant1.as_str(),
        ExampleCode::EqWarnMsg.as_str(),
        ExampleCode::LineVariant2.as_str(),
        ExampleCode::EqErrorMsg.as_str(),
        ExampleCode::LineVariant3.as_str()
    )
}

pub fn enum_example() -> String {
    let mut msg = ExampleCode::LineExample.as_str().to_owned();
    msg += ExampleCode::LineImpl.as_str();
    msg += ExampleCode::EqCrateIdx.as_str();
    msg += ExampleCode::EqModIdx.as_str();
    msg += ExampleCode::EqLocation.as_str();
    msg += ExampleCode::LineEnum.as_str();
    msg += "  ...\n}\n";

    msg
}

pub fn variant_example() -> String {
    let mut msg = ExampleCode::LineExample.as_str().to_owned();
    msg += ExampleCode::LineEnum.as_str();
    msg += ExampleCode::EqInfoMsg.as_str();
    msg += ExampleCode::LineVariant1.as_str();
    msg += "  ";
    msg += ExampleCode::EqWarnMsg.as_str();
    msg += ExampleCode::LineVariant2.as_str();
    msg += "  ";
    msg += ExampleCode::EqErrorMsg.as_str();
    msg += ExampleCode::LineVariant3.as_str();
    msg += "}\n";

    msg
}
