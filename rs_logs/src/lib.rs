use rs_logs_macro::Logger;

trait Logger {
    // fn log_path() -> (&'static str, &'static str);
}

#[derive(Logger)]
// --- syn::Meta::Path >> Invalid --- //
// #[info_msg]
// #[warn_msg]
// #[error_msg]
// #[crate_idx]
// #[mod_idx]
// #[location]
// --- syn::Meta::NameValue >> Invalid --- //
// #[info_msg = 15]
// #[warn_msg = 15]
// #[error_msg = 15]
// #[location = 15]
// #[crate_idx = "Example"]
// #[mod_idx = "Example"]
// #[crate_idx = 16]
// #[mod_idx = 256]
// --- syn::Meta::NameValue >> String --- //
#[info_msg = "Example"]
#[warn_msg = "Example"]
#[error_msg = "Example"]
#[crate_idx = 15]
#[mod_idx = 255]
#[location = "Example"]
// --- syn::Meta::List --- //
// #[warn_msg(23)] // syn::Meta::List
// #[warn_msg("a::b::c::d")] // syn::Meta::List
pub enum EnumLogger {
    Item1,
    Item2,
    Item3,
    Item4,
}

// #[derive(Logger)]
// pub struct StructLogger {
//     //
// }

// #[derive(Logger)]
// pub union UnionLogger {
//     a: u32,
// }
