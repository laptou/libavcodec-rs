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

    #[error("libav error {0}: {msg}", msg = libav_strerror(*.0))]
    Other(i32),

    #[error("the object could not be allocated")]
    Alloc,

    #[error("the string was not valid utf-8")]
    Utf8,

    #[error("the string contained a nul byte")]
    NulByte,

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

impl Error {
    pub fn new(code: i32) -> Self {
        if let Some(av_err) = constants::AVError::from_i32(code) {
            return Self::Av(av_err);
        }

        return Self::Other(code);
    }
}

fn libav_strerror(code: i32) -> String {
    let mut buffer = [0u8; 1024];

    unsafe { libavcodec_sys::av_strerror(code, buffer.as_mut_ptr() as _, buffer.len()) };

    String::from_utf8_lossy(&buffer).into_owned()
}

impl fmt::Display for AVError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "libav error {:?}: {}",
            self,
            libav_strerror(*self as i32)
        )
    }
}

impl std::error::Error for AVError {}
