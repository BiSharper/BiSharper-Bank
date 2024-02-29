use std::collections::HashMap;
use rfsa::{GFS_SEPARATOR, PathLike, ReadableVFile, ReadableVMetadata, VDirectory, VFile, VFileContainer, VFileSystem, VPath, VPathIterator, WritableVFile, WritableVMetadata};
use rfsa::error::{VFSError, VFSResult};
use crate::BankFileMeta;

pub type BankFile = VFile<BankFileMeta>;
pub type ReadableBankFile<'a> = ReadableVFile<'a, BankFileMeta>;
pub type WritableBankFile<'a> = WritableVFile<'a, BankFileMeta>;
pub type ReadableBankMetadata<'a> = ReadableVMetadata<'a, BankFileMeta>;
pub type WritableBankMetadata<'a> = WritableVMetadata<'a, BankFileMeta>;
pub type BankDirectory<'a> = VDirectory<'a, BankFileMeta, BankArchive>;

pub struct BankArchive { entries: HashMap<VPath, BankFile>, properties: HashMap<String, String> }


impl BankArchive {
    pub fn header_size(&self) -> usize { self.metadata_size() + self.version_size() }

    pub fn metadata_size(&self) -> usize { self.entries.keys().map(|path|
        path.as_bytes().len() + 21
    ).sum() }

    pub fn version_size(&self) -> usize { 22 + self.properties.iter().map(|(k, v) |
        k.as_bytes().len() + v.as_bytes().len() + 2
    ).sum::<usize>() }

    pub fn property_get(&self, name: &str) -> Option<&String> {
        self.properties.get(name)
    }

    pub fn property_insert(&mut self, name: &str, value: &str) -> Option<String> {
        self.properties.insert(name.to_string(), value.to_string())
    }

}

impl VFileSystem<BankFileMeta> for BankArchive {
    fn paths(&self) -> VFSResult<VPathIterator> {
        Ok(Box::new(self.entries.keys().cloned().collect::<Vec<VPath>>().into_iter()))
    }

    fn path_iter(&self, path_prefix: String, recursive: bool) -> VFSResult<VPathIterator> {
        let prefix_len = path_prefix.len();
        Ok(Box::new(self.entries.keys().filter( |candidate| {
            candidate.starts_with(path_prefix.as_str()) && (!recursive || !candidate[prefix_len..].contains(GFS_SEPARATOR))
        }).cloned().collect::<Vec<VPath>>().into_iter()))
    }
}


impl VFileContainer<BankFileMeta, BankArchive> for BankArchive {
    fn file_remove(&mut self, path: &VPath) -> VFSResult<VFile<BankFileMeta>> {
        self.entries.remove(path).ok_or(VFSError::EntryNotFound)
    }

    fn file_exists(&self, path: &VPath) -> VFSResult<bool> {
        Ok(self.entries.contains_key(path))
    }

    fn file_insert(&mut self, path: &VPath, file: VFile<BankFileMeta>) -> VFSResult<Option<BankFile>> {
        Ok(self.entries.insert(path.clone(), file))
    }

    fn file_mut(&mut self, path: &VPath) -> VFSResult<&mut VFile<BankFileMeta>> {
        self.entries.get_mut(path).ok_or(VFSError::EntryNotFound)
    }

    fn file_get(&self, path: &VPath) -> VFSResult<&VFile<BankFileMeta>> {
        self.entries.get(path).ok_or(VFSError::EntryNotFound)
    }

    fn dir_exists(&self, path: &VPath) -> VFSResult<bool> {
        Ok(self.entries.keys().find(|p| {
            p.starts_with(path.as_directory_string().as_str())
        }) != None)
    }
}


