use std::io::{Read, Seek};
use byteorder::{LittleEndian, ReadBytesExt};
use crate::{BankArchive, BankFileMeta, BankMime, BankReadOptions};
use crate::error::{BankError, BankResult, MissingTerminatorType};

trait PboReadExt: Read + Sized {
    fn read_bank_entry_name(&mut self, require_terminator: bool) -> BankResult<String> {
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

        Ok(String::from_utf8(Vec::from(bytes))?.to_lowercase())
    }

    fn read_entry_meta(&mut self, options: &BankReadOptions) -> BankResult<(String, BankFileMeta)> {
         Ok((
            self.read_bank_entry_name(options.respect_long_name_bug)?,
            BankFileMeta::create(
                self.read_i32::<LittleEndian>()?.into(),
                self.read_i32::<LittleEndian>()?,
                self.read_i32::<LittleEndian>()?,
                self.read_i32::<LittleEndian>()?,
                self.read_i32::<LittleEndian>()?,
            )
         ))
    }
}

impl<T: Read + Seek> PboReadExt for T  {

}

impl BankArchive {
    fn try_read<T: Read + Seek>(reader: &mut T, filename: &str, options: BankReadOptions) -> BankResult<BankArchive> {
        let start_pos = reader.stream_position()?;
        todo!()
    }

    fn read<T: Read + Seek>(reader: &mut T, filename: &str, options: BankReadOptions) -> BankArchive {
        Self::try_read(reader, filename, options).unwrap()
    }
}
