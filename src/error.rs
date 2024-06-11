use thiserror::Error;

#[derive(Error, Debug)]
pub enum DexError {
    #[error("binrw failed to parse something")]
    BinReadError(#[from] binrw::error::Error),
}
