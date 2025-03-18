mod codec;
mod constants;
mod error;
mod format;
mod frame;
mod packet;
mod rational;
mod stream;
mod swr;
mod sws;

pub use codec::*;
pub use constants::*;
pub use error::*;
pub use format::*;
pub use frame::*;
pub(crate) use libavcodec_sys as sys;
pub use packet::*;
pub use rational::*;
pub use stream::*;
pub use swr::*;
pub use sws::*;

#[cfg(feature = "tracing")]
mod tracing_support {
    use std::ffi::{CStr, c_void};

    use num_traits::FromPrimitive;

    use crate::AVLogLevel;

    use super::sys;

    extern "C" fn av_tracing_log_callback(
        _avcl: *mut c_void,
        level: i32,
        fmt: *const i8,
        args: *mut c_void,
    ) {
        let level = AVLogLevel::from_i32(level).unwrap();
        let level = match level {
            AVLogLevel::Panic => tracing::Level::ERROR,
            AVLogLevel::Fatal => tracing::Level::ERROR,
            AVLogLevel::Error => tracing::Level::ERROR,
            AVLogLevel::Warning => tracing::Level::WARN,
            AVLogLevel::Info => tracing::Level::INFO,
            AVLogLevel::Verbose => tracing::Level::DEBUG,
            AVLogLevel::Debug => tracing::Level::DEBUG,
            AVLogLevel::Trace => tracing::Level::TRACE,
        };

        let mut buffer = [0; 1024];
        let buffer_ptr = buffer.as_mut_ptr();

        unsafe {
            sys::avrs_format_msg(buffer_ptr, buffer.len() as i32, fmt, args as _);
        }

        let msg = unsafe { CStr::from_ptr(buffer_ptr).to_string_lossy() };
        let msg = msg.trim();

        match level {
            tracing::Level::ERROR => tracing::error!(target: "libavcodec", "{}", msg),
            tracing::Level::WARN => tracing::warn!(target: "libavcodec", "{}", msg),
            tracing::Level::INFO => tracing::info!(target: "libavcodec", "{}", msg),
            tracing::Level::DEBUG => tracing::debug!(target: "libavcodec", "{}", msg),
            tracing::Level::TRACE => tracing::trace!(target: "libavcodec", "{}", msg),
        }
    }

    pub fn setup_tracing() {
        unsafe {
            sys::av_log_set_level(AVLogLevel::Trace as i32);
            // weird shenanigans with casting b/c va_list type generates
            // different bindings on different platforms, so we just use *void
            // instead
            sys::av_log_set_callback(Some(std::mem::transmute(
                av_tracing_log_callback as *const (),
            )));
        }
    }
}

#[cfg(feature = "tracing")]
pub use tracing_support::setup_tracing;
