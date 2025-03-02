use crate::{AVCodecId, AVMediaType};
use crate::codec::CodecContext;
use crate::error::{FFmpegError, Result};
use crate::packet::Packet;
use crate::rational::Rational;
use libavcodec_sys as sys;
use num_traits::FromPrimitive;
use std::ffi::CString;
use std::path::Path;
use std::ptr;

pub struct FormatContext {
    inner: *mut sys::AVFormatContext,
}

impl FormatContext {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_str = path.as_ref().to_string_lossy();
        let path_cstr = CString::new(path_str.as_bytes()).unwrap();
        let mut inner = ptr::null_mut();

        let ret = unsafe {
            sys::avformat_open_input(
                &mut inner,
                path_cstr.as_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };

        if ret < 0 {
            return Err(FFmpegError::new(ret));
        }

        let ret = unsafe { sys::avformat_find_stream_info(inner, ptr::null_mut()) };
        if ret < 0 {
            unsafe { sys::avformat_close_input(&mut inner) };
            return Err(FFmpegError::new(ret));
        }

        Ok(FormatContext { inner })
    }

    pub fn as_mut_ptr(&mut self) -> *mut sys::AVFormatContext {
        self.inner
    }

    pub fn as_ptr(&self) -> *const sys::AVFormatContext {
        self.inner
    }

    pub fn streams(&self) -> impl Iterator<Item = Stream> {
        let nb_streams = unsafe { (*self.inner).nb_streams };
        let streams =
            unsafe { std::slice::from_raw_parts((*self.inner).streams, nb_streams as usize) };
        streams.iter().map(|&ptr| Stream { inner: ptr })
    }

    pub fn read_packet(&mut self, packet: &mut Packet) -> Result<bool> {
        let ret = unsafe { sys::av_read_frame(self.inner, packet.as_mut_ptr()) };
        if ret < 0 {
            if ret == sys::AVErrorEof {
                Ok(false)
            } else {
                Err(FFmpegError::new(ret))
            }
        } else {
            Ok(true)
        }
    }
}

impl Drop for FormatContext {
    fn drop(&mut self) {
        unsafe {
            sys::avformat_close_input(&mut self.inner);
        }
    }
}

pub struct Stream {
    inner: *mut sys::AVStream,
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

    pub fn codec_type(&self) -> AVMediaType {
        AVMediaType::from_i32(unsafe { (*(*self.inner).codecpar).codec_type }).unwrap()
    }

    pub fn codec_id(&self) -> AVCodecId {
        AVCodecId::from_i32(unsafe { (*(*self.inner).codecpar).codec_id }).unwrap()
    }

    pub fn apply_parameters_to_context(&self, codec_ctx: &mut CodecContext) -> Result<()> {
        let ret =
            unsafe { sys::avcodec_parameters_to_context(codec_ctx.as_mut_ptr(), self.codecpar()) };

        if ret < 0 {
            Err(FFmpegError::new(ret))
        } else {
            Ok(())
        }
    }
}
