use rs_logs_macro::Logger;

trait Logger {
    // fn log_path() -> (&'static str, &'static str);
}

#[derive(Logger)]
#[info_msg] // syn::Meta::Path
#[info_msg = 23] // syn::Meta::NameValue
#[info_msg = "a::b::c::d"] // syn::Meta::NameValue
#[warn_msg(23)] // syn::Meta::List
#[warn_msg("a::b::c::d")] // syn::Meta::List
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
