use std::{io, io::ErrorKind};

#[cfg(test)]
mod tests;

#[derive(Copy, Clone)]
pub(crate) enum OsError {
    Os(i32),
    Kind(ErrorKind),
}

impl From<io::Error> for OsError {
    fn from(value: io::Error) -> Self {
        match value.raw_os_error() {
            Some(e) => Self::Os(e),
            None => Self::Kind(value.kind()),
        }
    }
}

impl From<OsError> for io::Error {
    fn from(value: OsError) -> Self {
        match value {
            OsError::Os(e) => Self::from_raw_os_error(e),
            OsError::Kind(k) => k.into(),
        }
    }
}
