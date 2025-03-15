use crate::error::{FFmpegError, Result};
use libavcodec_sys as sys;

pub struct Packet {
    inner: *mut sys::AVPacket,
}

unsafe impl Send for Packet {}

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

    pub fn set_pts(&mut self, pts: i64) {
        unsafe { (*self.inner).pts = pts }
    }

    pub fn set_dts(&mut self, dts: i64) {
        unsafe { (*self.inner).dts = dts }
    }

    pub fn set_stream_index(&mut self, index: i32) {
        unsafe { (*self.inner).stream_index = index }
    }

    pub fn rescale_ts(&mut self, src_tb: sys::AVRational, dst_tb: sys::AVRational) {
        unsafe {
            sys::av_packet_rescale_ts(self.inner, src_tb, dst_tb);
        }
    }

    pub fn duration(&self) -> i64 {
        unsafe { (*self.inner).duration }
    }

    pub fn set_duration(&mut self, duration: i64) {
        unsafe { (*self.inner).duration = duration }
    }

    pub fn size(&self) -> i32 {
        unsafe { (*self.inner).size }
    }

    pub fn data(&self) -> *const u8 {
        unsafe { (*self.inner).data }
    }

    pub fn data_mut(&mut self) -> *mut u8 {
        unsafe { (*self.inner).data }
    }
}

impl Drop for Packet {
    fn drop(&mut self) {
        unsafe {
            sys::av_packet_free(&mut self.inner);
        }
    }
}
