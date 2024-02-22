use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use bisharper_filesystem::io::GfsFile;
use bisharper_filesystem::macros::GfsMeta;
use bisharper_filesystem::path::{GfsPath, OwnedGfsPath};

pub type BankFile = GfsFile<BankFileMeta>;
pub type BankPath<'a> = OwnedGfsPath<'a, BankFileMeta, BankArchive>;
pub const BANKMIME_DECOMPRESSED: i32 = 0x00000000;
pub const BANKMIME_COMPRESSED:   i32 = 0x43707273;
pub const BANKMIME_ENCRYPTED:    i32 = 0x456e6372;
pub const BANKMIME_VERSION:      i32 = 0x56657273;

#[derive(GfsMeta, Copy, Clone, Default)]
pub struct BankFileMeta {
    mime:           i32,
    content_length: u32,
    buffer_offset:  u64,
    timestamp:      u64,
    buffer_length:  u32
}

enum BankEntry {
    Cached(BankFile),
    Untouched(BankFileMeta)
}

pub struct BankArchive {
    buffer:  Arc<[u8]>,
    entries: Arc<RwLock<HashMap<GfsPath, BankEntry>>>,
    properties: Arc<RwLock<HashMap<String, String>>>
}

impl BankEntry {
    fn metadata(&self) -> &BankFileMeta {
        match self {
            BankEntry::Cached(c) => c.metadata(),
            BankEntry::Untouched(m) => m
        }
    }
}

