
use std::io::{Read, Seek};
use crate::{BankArchive, BankReadOptions};
use crate::error::BankResult;


impl BankArchive {
    fn try_read<T: Seek + Read>(reader: T, filename: &str, options: BankReadOptions) -> BankResult<BankArchive> {
        todo!()
    }

    fn read<T: Seek + Read>(reader: T, filename: &str, options: BankReadOptions) -> BankArchive {
        Self::try_read(reader, filename, options).unwrap()
    }
}