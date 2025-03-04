use crate::AVCodecId;
use crate::AVDiscard;
use crate::AVPixelFormat;
use crate::error::{FFmpegError, Result};
use crate::frame::Frame;
use crate::packet::Packet;
use libavcodec_sys as sys;
use num_traits::FromPrimitive;
use std::ptr;

pub struct Codec {
    inner: *const sys::AVCodec,
}

unsafe impl Send for Codec {}

impl Codec {
    pub fn find_decoder(id: AVCodecId) -> Option<Self> {
        let inner = unsafe { sys::avcodec_find_decoder(id as i32) };
        if inner.is_null() {
            None
        } else {
            Some(Codec { inner })
        }
    }

    pub fn as_ptr(&self) -> *const sys::AVCodec {
        self.inner
    }
}

pub struct CodecContext {
    inner: *mut sys::AVCodecContext,
}

unsafe impl Send for CodecContext {}

impl CodecContext {
    pub fn new(codec: &Codec) -> Result<Self> {
        let inner = unsafe { sys::avcodec_alloc_context3(codec.as_ptr()) };
        if inner.is_null() {
            return Err(FFmpegError::new(-1));
        }
        Ok(CodecContext { inner })
    }

    pub fn as_mut_ptr(&mut self) -> *mut sys::AVCodecContext {
        self.inner
    }

    pub fn as_ptr(&self) -> *const sys::AVCodecContext {
        self.inner
    }

    pub fn width(&self) -> usize {
        unsafe { (*self.inner).width as usize }
    }

    pub fn height(&self) -> usize {
        unsafe { (*self.inner).height as usize }
    }

    pub fn pix_fmt(&self) -> AVPixelFormat {
        AVPixelFormat::from_i32(unsafe { (*self.inner).pix_fmt }).unwrap()
    }

    pub fn set_skip_frame(&mut self, value: AVDiscard) {
        unsafe {
            (*self.inner).skip_frame = value as i32;
        }
    }

    pub fn open(&mut self, codec: &Codec) -> Result<()> {
        let ret = unsafe { sys::avcodec_open2(self.inner, codec.as_ptr(), ptr::null_mut()) };
        if ret < 0 {
            Err(FFmpegError::new(ret))
        } else {
            Ok(())
        }
    }

    pub fn send_packet(&mut self, packet: &Packet) -> Result<()> {
        let ret = unsafe { sys::avcodec_send_packet(self.inner, packet.as_ptr()) };
        if ret < 0 {
            Err(FFmpegError::new(ret))
        } else {
            Ok(())
        }
    }

    pub fn receive_frame(&mut self, frame: &mut Frame) -> Result<()> {
        let ret = unsafe { sys::avcodec_receive_frame(self.inner, frame.as_mut_ptr()) };
        if ret < 0 {
            Err(FFmpegError::new(ret))
        } else {
            Ok(())
        }
    }
}

impl Drop for CodecContext {
    fn drop(&mut self) {
        unsafe {
            sys::avcodec_free_context(&mut self.inner);
        }
    }
}
