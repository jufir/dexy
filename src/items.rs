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

#[derive(Debug, PartialEq, BinRead)]
pub struct FieldID {
    pub class_idx: u16,
    pub type_idx: u16,
    pub name_idx: u32,
}

impl FieldID {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}

#[derive(Debug, PartialEq, BinRead)]
pub struct MethodID {
    pub class_idx: u16,
    pub proto_idx: u16,
    pub name_idx: u32,
}

impl MethodID {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}

#[derive(Debug, PartialEq, BinRead)]
pub struct ClassDef {
    pub class_idx: u32,
    pub access_flags: u32,
    pub superclass_idx: u32,
    pub interfaces_off: u32,
    pub source_file_idx: u32,
    pub annotations_off: u32,
    pub class_data_off: u32,
    pub static_values_off: u32,
}

impl ClassDef {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
