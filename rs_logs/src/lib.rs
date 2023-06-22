use rs_logs_macro::Logger;

trait Logger {
    // fn log_path() -> (&'static str, &'static str);
}

// #[derive(Logger)]
// enum ErrLog {
//     A,
// }

#[derive(Logger)]
// #[info_msg] //  Meta::Path
// #[warn_msg] //  Meta::Path
// #[error_msg] // Meta::Path
// #[config] // Meta::Path
#[info_msg("hot")] // Meta::List
#[warn_msg(a = 2, b = 3)] // Meta::List
#[config(2, 34, "lorem ipsum dorem", rs_logger::debug::this)]
// #[error_msg = "Some Data"]
pub enum DevEnum {
    Item1,
    Item2,
    Item3,
    Item4,
}

// #[derive(Logger)]
// #[config(
//     crate_idx = 1,
//     mod_idx = 1,
//     location = "rs_logger::module::path::can::be::long"
// )]
// pub enum Logs101 {
//     #[info_msg("Log some content")]
//     _Log01,
//     #[warn_msg("A {} warning caused by {}")]
//     _Warn02(i32, String),
//     #[error_msg("Error caused by {}")]
//     _Error03(String),
//     #[info_msg("Struct data")]
//     _Info04 { x: i32 },
// }

#[derive(Logger)]
#[crate_idx = 1]
#[mod_idx = 2]
#[location = "rs_logger::module::path"]
pub enum TestLog {
    Item1,
}
