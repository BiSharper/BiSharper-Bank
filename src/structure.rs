use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use bisharper_filesystem::io::GfsFile;
use bisharper_filesystem::macros::GfsMeta;
use bisharper_filesystem::path::{GfsPath, OwnedGfsPath};
use crate::mime::BankMime;

pub type BankFile = GfsFile<BankFileMeta>;
pub type BankPath<'a> = OwnedGfsPath<'a, BankFileMeta, BankArchive>;

#[derive(GfsMeta, Copy, Clone, Default)]
pub struct BankFileMeta {
    mime:           BankMime,
    content_length: u32,
    buffer_offset:  u64,
    timestamp:      u64,
    buffer_length:  u32
}

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

