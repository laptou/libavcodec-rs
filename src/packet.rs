use std::ptr::NonNull;

use crate::error::{Error, Result};
use libavcodec_sys as sys;

#[derive(Debug)]
pub struct Packet {
    inner: NonNull<sys::AVPacket>,
}

unsafe impl Send for Packet {}

impl AsRef<sys::AVPacket> for Packet {
    fn as_ref(&self) -> &sys::AVPacket {
        unsafe { self.inner.as_ref() }
    }
}

impl AsMut<sys::AVPacket> for Packet {
    fn as_mut(&mut self) -> &mut sys::AVPacket {
        unsafe { self.inner.as_mut() }
    }
}

impl Packet {
    pub fn new() -> Result<Self> {
        let inner = unsafe { sys::av_packet_alloc() };
        let inner = NonNull::new(inner).ok_or(Error::Alloc)?;

        Ok(Packet { inner })
    }

    pub fn stream_index(&self) -> i32 {
        self.as_ref().stream_index
    }

    pub fn pts(&self) -> i64 {
        self.as_ref().pts
    }

    pub fn dts(&self) -> i64 {
        self.as_ref().dts
    }

    pub fn set_pts(&mut self, pts: i64) {
        self.as_mut().pts = pts
    }

    pub fn set_dts(&mut self, dts: i64) {
        self.as_mut().dts = dts
    }

    pub fn set_stream_index(&mut self, index: i32) {
        self.as_mut().stream_index = index
    }

    pub fn rescale_ts(&mut self, src_tb: sys::AVRational, dst_tb: sys::AVRational) {
        unsafe {
            sys::av_packet_rescale_ts(self.as_mut(), src_tb, dst_tb);
        }
    }

    pub fn duration(&self) -> i64 {
        self.as_ref().duration
    }

    pub fn set_duration(&mut self, duration: i64) {
        self.as_mut().duration = duration
    }

    pub fn size(&self) -> i32 {
        self.as_ref().size
    }

    pub fn data(&self) -> *const u8 {
        self.as_ref().data
    }

    pub fn data_mut(&mut self) -> *mut u8 {
        self.as_mut().data
    }

    pub fn unref(&mut self) {
        unsafe { sys::av_packet_unref(self.inner.as_ptr()) }
    }
}

impl Drop for Packet {
    fn drop(&mut self) {
        unsafe {
            sys::av_packet_free(&mut self.inner.as_ptr());
        }
    }
}
