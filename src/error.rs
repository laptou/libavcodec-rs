use std::fmt;

use num_traits::FromPrimitive;
use thiserror::Error;

use crate::AVError;
use crate::constants;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Av(AVError),

    #[error(transparent)]
    Io(std::io::Error),

    #[error("the object could not be allocated")]
    Alloc,

    #[error("the string was not valid utf-8")]
    Utf8,

    #[error("the string contained a nul byte")]
    NulByte,
}

impl Error {
    pub fn new(code: i32) -> Self {
        if let Some(av_err) = constants::AVError::from_i32(code) {
            return Self::Av(av_err);
        }

        return Self::Io(std::io::Error::from_raw_os_error(-code));
    }
}

impl fmt::Display for AVError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = unsafe {
            let mut buffer = [0i8; 1024];

            libavcodec_sys::av_strerror(*self as i32, buffer.as_mut_ptr(), buffer.len());

            String::from_utf8_lossy(
                &buffer
                    .iter()
                    .take_while(|&&c| c != 0)
                    .map(|&c| c as u8)
                    .collect::<Vec<_>>(),
            )
            .into_owned()
        };

        write!(f, "libav error {:?}: {}", self, message)
    }
}

impl std::error::Error for AVError {}
