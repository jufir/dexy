use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

pub use error::DexError;
pub use items::StringID;

pub mod error;
pub mod header;
pub mod items;

mod parse_utils;

#[derive(Debug, PartialEq, BinRead)]
pub struct Dex {
    pub header: header::Header,
    #[br(count = header.string_ids_size)]
    pub string_ids: Vec<StringID>,
}

impl Dex {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, DexError> {
        Ok(reader.read_le()?)
    }
}
