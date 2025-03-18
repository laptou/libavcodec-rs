use std::error::Error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, FFmpegError>;

#[derive(Debug)]
pub struct FFmpegError {
    pub code: i32,
    pub message: String,
}

impl FFmpegError {
    pub fn new(code: i32) -> Self {
        let message = unsafe {
            let mut buffer = [0i8; 1024];

            libavcodec_sys::av_strerror(code, buffer.as_mut_ptr(), buffer.len());

            String::from_utf8_lossy(
                &buffer
                    .iter()
                    .take_while(|&&c| c != 0)
                    .map(|&c| c as u8)
                    .collect::<Vec<_>>(),
            )
            .into_owned()
        };

        FFmpegError { code, message }
    }
}

impl fmt::Display for FFmpegError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FFmpeg error {}: {}", self.code, self.message)
    }
}

impl Error for FFmpegError {}
