use crate::error::{FFmpegError, Result};
use libavcodec_sys as sys;
use std::ptr;

pub struct Packet {
    inner: *mut sys::AVPacket,
}

impl Packet {
    pub fn new() -> Result<Self> {
        let inner = unsafe { sys::av_packet_alloc() };
        if inner.is_null() {
            return Err(FFmpegError::new(-1));
        }
        Ok(Packet { inner })
    }

    pub fn as_mut_ptr(&mut self) -> *mut sys::AVPacket {
        self.inner
    }

    pub fn as_ptr(&self) -> *const sys::AVPacket {
        self.inner
    }

    pub fn stream_index(&self) -> i32 {
        unsafe { (*self.inner).stream_index }
    }

    pub fn pts(&self) -> i64 {
        unsafe { (*self.inner).pts }
    }

    pub fn dts(&self) -> i64 {
        unsafe { (*self.inner).dts }
    }

    pub fn unref(&mut self) {
        unsafe { sys::av_packet_unref(self.inner) }
    }
}

impl Drop for Packet {
    fn drop(&mut self) {
        unsafe {
            sys::av_packet_free(&mut self.inner);
        }
    }
} 
