mod traits;
pub use traits::Logger;

#[derive(Logger)]
// --- ? --- //
// #[info_msg]
// #[warn_msg]
// #[error_msg]
// #[crate_idx]
// #[mod_idx]
// #[location]
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
