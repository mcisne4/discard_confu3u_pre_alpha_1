use rs_logs_macro::Logger;

trait Logger {}

#[derive(Logger)]
#[idx(crate_idx = 1, mod_idx = 1)]
#[src("rs_logger::module::path")]
enum Logs101 {
    #[log("Log some content")]
    _Log01,
    #[warning("A {} warning caused by {}")]
    _Warn02(i32, String),
    #[error("Error caused by {}")]
    _Error03(String),
}
