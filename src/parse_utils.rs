use binrw::{io::Read, BinRead, BinResult};

#[binrw::parser(reader, endian)]
pub fn parse_version() -> BinResult<u32> {
    let version_str = parse_null_terminated_string(reader, endian, ())?;
    let version: u32 = version_str.parse().unwrap_or(0);

    Ok(version)
}

#[binrw::parser(reader)]
pub fn parse_null_terminated_string() -> BinResult<String> {
    Ok(String::from_utf8_lossy(
        reader
            .bytes()
            .filter_map(Result::ok)
            .take_while(|&b| b != b'\0')
            .collect::<Vec<u8>>()
            .as_slice(),
    )
    .to_string())
}
