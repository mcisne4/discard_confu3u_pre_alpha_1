mod traits;
pub use traits::Logger;

#[derive(Logger)]
// --- Not Available on Enum Definition --- //
// #[info_msg]
// #[warn_msg]
// #[error_msg]
// --- No Value --- //
// #[crate_idx]
// #[mod_idx]
#[location]
// --- Invalid Equals Value --- //
// #[crate_idx = "Hello World"]
// #[mod_idx = "Hello World Again"]
// #[crate_idx = 16]
// #[mod_idx = 255]
// #[location = true]
// #[location = 'c']
// --- Valid Equals Value --- //
// #[crate_idx = 15]
// #[mod_idx = 255]
// #[location = "Hello World"]
// --- Invalid Group Value --- //
// #[crate_idx()]
// #[crate_idx(a = true, b = 23)]
// #[crate_idx("ABC")]
// #[crate_idx(16)]
// #[mod_idx()]
// #[mod_idx(a = true, b = 23)]
// #[mod_idx("ABC")]
// #[mod_idx(16)]
// --- Valid Group Value --- //
#[crate_idx(15)]
#[mod_idx(255)]
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
