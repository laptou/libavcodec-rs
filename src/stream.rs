use std::ptr::NonNull;
use std::time::Duration;

use libavcodec_sys as sys;
use num_traits::FromPrimitive;

use crate::{AVCodecId, AVMediaType, AVSampleFormat, CodecContext, Error, Rational};

pub struct Stream {
    pub(crate) inner: NonNull<sys::AVStream>,
}

unsafe impl Send for Stream {}

impl AsRef<sys::AVStream> for Stream {
    fn as_ref(&self) -> &sys::AVStream {
        unsafe { self.inner.as_ref() }
    }
}

impl AsMut<sys::AVStream> for Stream {
    fn as_mut(&mut self) -> &mut sys::AVStream {
        unsafe { self.inner.as_mut() }
    }
}

impl Stream {
    pub fn as_mut_ptr(&mut self) -> *mut sys::AVStream {
        unsafe { self.inner.as_mut() }
    }

    pub fn as_ptr(&self) -> *const sys::AVStream {
        self.inner.as_ptr()
    }

    pub fn index(&self) -> i32 {
        self.as_ref().index
    }

    pub fn codecpar(&mut self) -> &mut sys::AVCodecParameters {
        unsafe { &mut *self.as_mut().codecpar }
    }

    pub fn time_base(&self) -> Rational {
        self.as_ref().time_base.into()
    }

    pub fn duration_ts(&self) -> i64 {
        self.as_ref().duration
    }

    pub fn start_time(&self) -> i64 {
        self.as_ref().start_time
    }

    pub fn duration(&self) -> Option<Duration> {
        let time_base = self.time_base();
        let duration_ts = self.duration_ts();

        if duration_ts < 0 {
            return None;
        }

        Some(Duration::from_secs_f64(duration_ts as f64 * time_base.as_f64()))
    }

    pub fn codec_type(&self) -> AVMediaType {
        AVMediaType::from_i32(unsafe { (*self.as_ref().codecpar).codec_type }).unwrap()
    }

    pub fn codec_id(&self) -> AVCodecId {
        AVCodecId::from_i32(unsafe { (*self.as_ref().codecpar).codec_id as i32 }).unwrap()
    }

    pub fn apply_parameters_to_context(
        &mut self,
        codec_ctx: &mut CodecContext,
    ) -> crate::Result<()> {
        let ret =
            unsafe { sys::avcodec_parameters_to_context(codec_ctx.as_mut_ptr(), self.codecpar()) };

        if ret < 0 {
            Err(Error::new(ret))
        } else {
            Ok(())
        }
    }

    pub fn set_audio_codec_params(
        &mut self,
        codec_type: AVMediaType,
        codec_id: AVCodecId,
        sample_rate: usize,
        channels: usize,
        sample_fmt: AVSampleFormat,
    ) -> crate::Result<()> {
        unsafe {
            let codecpar = self.codecpar();
            codecpar.codec_type = codec_type as i32;
            codecpar.codec_id = codec_id as _;
            codecpar.sample_rate = sample_rate as i32;
            codecpar.format = sample_fmt as i32;
            codecpar.bit_rate = (sample_rate * 16) as i64; // 16 bits per sample
            sys::av_channel_layout_default(&mut codecpar.ch_layout, channels as i32);

            // Set stream time base
            self.as_mut().time_base = sys::AVRational {
                num: 1,
                den: sample_rate as i32,
            };
        }

        Ok(())
    }
}
