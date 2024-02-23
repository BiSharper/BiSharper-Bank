mod structure; pub use structure::*;
mod mime; pub use mime::*;
mod version; pub use version::*;
mod read;
mod error;


pub const FILE_BANK_VERSION: &'static str = env!("CARGO_PKG_VERSION");



