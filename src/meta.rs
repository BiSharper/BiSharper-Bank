use bisharper_filesystem::macros::GfsMeta;

#[derive(GfsMeta, Copy, Clone, Default)]
pub struct BankFileMeta {
    mime:           BankMime,
    content_length: i32,
    buffer_offset:  u64,
    timestamp:      i32,
    buffer_length:  i32
}

impl BankFileMeta {
    pub fn create(mime: BankMime, content_length: i32, buffer_offset: i32, timestamp: i32, buffer_length: i32) -> Self {
        BankFileMeta {
            mime,
            content_length ,
            buffer_offset: buffer_offset as u64,
            timestamp,
            buffer_length,
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