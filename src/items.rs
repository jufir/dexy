use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct StringID {
    pub string_data_off: u32,
}

impl StringID {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}

#[derive(Debug, PartialEq, BinRead)]
pub struct TypeID {
    pub descriptor_idx: u32,
}

impl TypeID {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}

#[derive(Debug, PartialEq, BinRead)]
pub struct ProtoID {
    pub shorty_idx: u32,
    pub return_type_idx: u32,
    pub parameters_off: u32,
}

impl ProtoID {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
