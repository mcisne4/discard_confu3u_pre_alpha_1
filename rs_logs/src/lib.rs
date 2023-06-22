use rs_logs_macro::Logger;

trait Logger {
    // fn log_path() -> (&'static str, &'static str);
}

#[derive(Logger)]
// #[info_msg] //  Meta::Path
// #[warn_msg] //  Meta::Path
// #[error_msg] // Meta::Path
// #[config] // Meta::Path
// #[info_msg("hot")] // Meta::List
// #[warn_msg(a = 2, b = 3)] // Meta::List
// #[error_msg = "Some Data"]
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
