use std::time::Duration;

use libavcodec_sys as sys;
use num_traits::FromPrimitive;

use crate::{AVCodecId, AVMediaType, CodecContext, FFmpegError, Rational};

pub struct Stream {
    pub(crate) inner: *mut sys::AVStream,
}

impl Stream {
    pub fn index(&self) -> i32 {
        unsafe { (*self.inner).index }
    }

    pub fn codecpar(&self) -> *mut sys::AVCodecParameters {
        unsafe { (*self.inner).codecpar }
    }

    pub fn time_base(&self) -> Rational {
        unsafe { (*self.inner).time_base.into() }
    }

    pub fn duration_ts(&self) -> i64 {
        unsafe { (*self.inner).duration }
    }

    pub fn start_time(&self) -> i64 {
        unsafe { (*self.inner).start_time }
    }
    
    pub fn duration(&self) -> Duration {
        let time_base = self.time_base();
        let duration_ts = self.duration_ts();
        Duration::from_secs(duration_ts as u64) * time_base.num() as u32 / time_base.den() as u32
    }

    pub fn codec_type(&self) -> AVMediaType {
        AVMediaType::from_i32(unsafe { (*(*self.inner).codecpar).codec_type }).unwrap()
    }

    pub fn codec_id(&self) -> AVCodecId {
        AVCodecId::from_i32(unsafe { (*(*self.inner).codecpar).codec_id }).unwrap()
    }

    pub fn apply_parameters_to_context(&self, codec_ctx: &mut CodecContext) -> crate::Result<()> {
        let ret =
            unsafe { sys::avcodec_parameters_to_context(codec_ctx.as_mut_ptr(), self.codecpar()) };

        if ret < 0 {
            Err(FFmpegError::new(ret))
        } else {
            Ok(())
        }
    }
}
