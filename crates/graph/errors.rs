use thiserror::Error;

#[derive(Error, Debug)]
pub enum EszipError {
    #[error("unsupported powerbase eszip version (expected {expected:?}, found {found:?})")]
    UnsupportedVersion {
        expected: &'static [u8],
        found: Option<Vec<u8>>,
    },
}
