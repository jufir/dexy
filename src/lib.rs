use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

pub use error::DexError;

use items::{ClassDef, FieldID, MethodID, ProtoID, StringID, TypeID};

pub mod error;
pub mod header;
pub mod items;

mod parse_utils;

#[derive(Debug, PartialEq, BinRead)]
pub struct Dex {
    pub header: header::Header,

    #[br(count = header.string_ids_size)]
    pub string_ids: Vec<StringID>,

    #[br(count = header.type_ids_size)]
    pub type_ids: Vec<TypeID>,

    #[br(count = header.proto_ids_size)]
    pub proto_ids: Vec<ProtoID>,

    #[br(count = header.field_ids_size)]
    pub field_ids: Vec<FieldID>,

    #[br(count = header.method_ids_size)]
    pub method_ids: Vec<MethodID>,

    #[br(count = header.class_defs_size)]
    pub class_defs: Vec<ClassDef>,

    #[br(count = header.data_size)]
    pub data: Vec<u8>,

    #[br(count = header.link_size)]
    pub link_data: Vec<u8>,
}

impl Dex {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, DexError> {
        Ok(reader.read_le()?)
    }
}
