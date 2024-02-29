use rfsa::macros::VMeta;

#[derive(VMeta, Copy, Clone, Default, Eq, PartialEq)]
pub struct BankFileMeta {
    pub fully_loaded:   bool,
    pub mime:           BankMime,
    pub content_length: u32,
    pub buffer_offset:  u64,
    pub timestamp:      u32,
    pub buffer_length:  u32
}


impl BankFileMeta {
    pub fn create(fully_loaded: bool, mime: BankMime, content_length: u32, buffer_offset: u32, timestamp: u32, buffer_length: u32) -> Self {
        BankFileMeta {
            fully_loaded, mime, content_length,
            buffer_offset: buffer_offset as u64,
            timestamp, buffer_length,
        }
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum BankMime {
    Version,
    #[default]
    Decompressed,
    Compressed,
    Encrypted,
    Other(i32)
}

impl std::fmt::Display for BankMime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BankMime::Version => write!(f, "Version"),
            BankMime::Decompressed => write!(f, "Decompressed"),
            BankMime::Compressed => write!(f, "Compressed"),
            BankMime::Encrypted => write!(f, "Encrypted"),
            BankMime::Other(unknown) => write!(f, "Unknown ({:#08x})", unknown)
        }
    }
}

impl<T: Into<i32>> From<T> for BankMime {
    fn from(value: T) -> Self {
        match value.into() {
            0x00000000 => Self::Decompressed,
            0x43707273 => Self::Compressed,
            0x456e6372 => Self::Encrypted,
            0x56657273 => Self::Version,
            other => Self::Other(other)
        }
    }
}