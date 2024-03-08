use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use rfsa::{ReadableVFile, ReadableVMetadata, VDirectory, VFile, VFileSystem, VPath, WritableVFile, WritableVMetadata};
use rfsa::impls::memory::MemoryFileSystem;

use crate::BankFileMeta;

pub type BankFile = VFile<BankFileMeta>;
pub type ReadableBankFile = ReadableVFile<BankFileMeta>;
pub type WritableBankFile<'a> = WritableVFile<'a, BankFileMeta, BankArchive>;
pub type ReadableBankMetadata = ReadableVMetadata<BankFileMeta>;
pub type WritableBankMetadata<'a> = WritableVMetadata<'a, BankFileMeta, BankArchive>;
pub type BankDirectory<'a> = VDirectory<'a, BankFileMeta, BankArchive>;


pub struct BankArchive {
    file_system: MemoryFileSystem<BankFileMeta>,
    properties: HashMap<String, String>
}

impl Deref for BankArchive {
    type Target = MemoryFileSystem<BankFileMeta>;

    fn deref(&self) -> &Self::Target { &self.file_system }
}

impl DerefMut for BankArchive {
    fn deref_mut(&mut self) -> &mut Self::Target  { &mut self.file_system }
}

#[allow(dead_code)]
impl BankArchive {
    pub fn header_size(&self) -> rfsa::Result<usize> { Ok(self.metadata_size()? + self.version_size()) }

    pub fn metadata_size(&self) -> rfsa::Result<usize> { Ok(self.fs_iter()?.map(|path|
        path.as_bytes().len() + 21
    ).sum()) }

    pub fn version_size(&self) -> usize { 22 + self.properties.iter().map(|(k, v) |
        k.as_bytes().len() + v.as_bytes().len() + 2
    ).sum::<usize>() }

    pub fn property_get(&self, name: &str) -> Option<&String> { self.properties.get(name) }

    pub fn property_insert(&mut self, name: &str, value: &str) -> Option<String> {
        self.properties.insert(name.to_string(), value.to_string())
    }

    pub fn create(root: VPath, entries: HashMap<VPath, BankFile>, properties: HashMap<String, String>) -> Self {
        Self { file_system: MemoryFileSystem::new(root, entries), properties, }
    }

}


