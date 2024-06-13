use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::parse_utils;

#[derive(Debug, PartialEq, BinRead)]
#[br(magic = b"dex\n")]
pub struct Header {
    #[br(parse_with = parse_utils::parse_version)]
    pub version: u32,
    pub checksum: u32,

    #[br(count = 20)]
    pub signature: Vec<u8>,

    pub file_size: u32,
    pub header_size: u32,
    pub endian_tag: EndianTag,
    pub link_size: u32,
    pub link_off: u32,
    pub map_off: u32,
    pub string_ids_size: u32,
    pub string_ids_off: u32,
    pub type_ids_size: u32,
    pub type_ids_off: u32,
    pub proto_ids_size: u32,
    pub proto_ids_off: u32,
    pub field_ids_size: u32,
    pub field_ids_off: u32,
    pub method_ids_size: u32,
    pub method_ids_off: u32,
    pub class_defs_size: u32,
    pub class_defs_off: u32,
    pub data_size: u32,
    pub data_off: u32,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum EndianTag {
    #[br(magic = b"\x12\x34\x56\x78")]
    EndianConstant,

    #[br(magic = b"\x78\x56\x34\x12")]
    ReverseEndianConstant,
}

impl Header {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
