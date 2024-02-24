
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use bisharper_filesystem::io::GfsFile;
use bisharper_filesystem::path::{GfsPath, OwnedGfsPath};
mod meta; pub use meta::*;
mod options; pub use options::*;
pub mod error;
pub mod io;


pub const FILE_BANK_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub type BankFile = GfsFile<BankFileMeta>;
pub type BankPath<'a> = OwnedGfsPath<'a, BankFileMeta, BankArchive>;


#[derive(Clone)]
enum BankEntry {
    Cached(BankFile),
    Untouched(BankFileMeta)
}

pub struct BankArchive {
    pub(crate) buffer:  Arc<[u8]>,
    pub(crate) entries: Arc<RwLock<HashMap<GfsPath, BankEntry>>>,
    pub(crate) properties: Arc<RwLock<HashMap<String, String>>>
}

impl BankEntry {
    fn metadata(&self) -> &BankFileMeta {
        match self {
            BankEntry::Cached(c) => c.metadata(),
            BankEntry::Untouched(m) => m
        }
    }
}


