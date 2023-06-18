use rs_logs_macro::Logger;

trait Logger {
    fn log_path() -> (&'static str, &'static str);
}

#[derive(Logger)]
#[config(
    crate_idx = 1,
    mod_idx = 1,
    location = "rs_logger::module::path::can::be::long"
)]
enum Logs101 {
    #[info_msg("Log some content")]
    _Log01,
    #[warn_msg("A {} warning caused by {}")]
    _Warn02(i32, String),
    #[error_msg("Error caused by {}")]
    _Error03(String),
}
