use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};
use std::sync::Arc;
use byteorder::{LittleEndian, ReadBytesExt};
use rfsa::{PathLike, VPath};
use crate::{BankArchive, BankFile, BankFileMeta, BankMime, BankReadOptions};
use crate::error::{BankError, BankResult, MissingTerminatorType};

trait PboReadExt: Read + Sized {
    fn read_bank_string(&mut self, require_terminator: bool) -> BankResult<String> {
        let mut bytes: [u8; 1024] = [0; 1024];

        for i in 0..1024 {
            let next = self.read_u8()?;
            bytes[i] = next;
            if next == 0 {
                break;
            }
        }
        if require_terminator && (bytes[1023] != 0) {
            return Err(BankError::MissingNullTerminator(MissingTerminatorType::String))
        } else {
            bytes[1023] = 0;
        }

        Ok(String::from_utf8(Vec::from(bytes))?)
    }

    fn read_bank_entry_meta(&mut self, options: &BankReadOptions) -> BankResult<(String, BankFileMeta)> {
        let name =  self.read_bank_string(options.respect_long_name_bug)?.to_lowercase();
        let mime: BankMime = self.read_i32::<LittleEndian>()?.into();
        let fully_loaded = mime == BankMime::Compressed && !options.decompress_lazily;
        Ok((name,
            BankFileMeta::create(
                fully_loaded,
                mime,
                self.read_u32::<LittleEndian>()?,
                self.read_u32::<LittleEndian>()?,
                self.read_u32::<LittleEndian>()?,
                self.read_u32::<LittleEndian>()?,
            )
         ))
    }

    fn read_bank_properties(&mut self, properties: &mut HashMap<String, String>, options: &BankReadOptions) -> BankResult<()>{
        loop {
            let name = self.read_bank_string(options.respect_long_name_bug)?;
            if name.len() == 0 {
                return Ok(());
            }
            properties.insert(name, self.read_bank_string(options.respect_long_name_bug)?);
        }
    }
}

impl<T: Read + Seek> PboReadExt for T  {

}

#[allow(dead_code)]
impl BankArchive {
    fn try_read<T: Read + Seek>(reader: &mut T, filename: &str, options: &BankReadOptions) -> BankResult<BankArchive> {
        let (root, entries, properties) = {
            let mut entry_index: i32 = 0;
            let mut start_offset: i32 = 0;
            let mut properties: HashMap<String, String> = HashMap::new();
            let mut metas: HashMap<String, BankFileMeta> = HashMap::new();
            let mut encountered_version: bool = false;
            loop {
                let (name, mut meta) = reader.read_bank_entry_meta(&options)?;
                if meta.mime == BankMime::Version {
                    if entry_index == 0 && meta.mime == BankMime::Version && name.len() == 0 {
                        reader.read_bank_properties(&mut properties, options)?;
                        encountered_version = true;
                    } else if !options.allow_post_mature_version {
                        return Err(BankError::PostMatureVersion);
                    }
                }

                if entry_index == 0 && !encountered_version && options.requires_version_header() {
                    return Err(BankError::MissingVersionEntry);
                }

                if meta.buffer_offset == 0 && meta.mime == BankMime::Decompressed && name.len() == 0 &&
                    meta.buffer_length == 0 && meta.timestamp == 0 && meta.content_length == 0 {
                    break;
                }

                meta.buffer_offset = start_offset as u32 as u64;
                let entry_length = meta.buffer_length as i32;
                if !options.respect_signedness_bug && entry_length < 0 {
                    return Err(BankError::InvalidDataOffset);
                }
                start_offset += entry_length;

                entry_index += 1;
                if meta.content_length >= options.valid_entry_threshold {
                    metas.insert(name, meta);
                }
            }
            let root = match properties.get("prefix") {
                None => VPath::normalized(filename),
                Some(p) => VPath::normalized(&*p)
            };
            let buffer_start = reader.stream_position()?;

            let entries: HashMap<VPath, BankFile> = metas.drain().map(|(name, mut entry)| {
                entry.buffer_offset += buffer_start;
                reader.seek(SeekFrom::Start(entry.buffer_offset)).unwrap();
                let mut data: Vec<u8> = vec![0; entry.buffer_length as usize];
                reader.read_exact(&mut *data).unwrap();
                return (VPath::normalized(&*name), BankFile::create(entry, Arc::from(data.as_slice())));
            }).collect();

            (root, entries, properties)
        };
        Ok(BankArchive::create(root, entries, properties))
    }


    fn read<T: Read + Seek>(reader: &mut T, filename: &str, options: &BankReadOptions) -> BankArchive {
        Self::try_read(reader, filename, options).unwrap()
    }
}
