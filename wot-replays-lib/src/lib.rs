pub mod dictionaries;
pub mod helpers;
pub mod models;

pub use helpers::*;

pub fn get_version() -> String {
    format!(
        "{}.{}.{}{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH"),
        option_env!("CARGO_PKG_VERSION_PRE").unwrap_or("")
    )
}
