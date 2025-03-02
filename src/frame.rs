use crate::error::{FFmpegError, Result};
use libavcodec_sys as sys;
use std::slice;

pub struct Frame {
    inner: *mut sys::AVFrame,
}

impl Frame {
    pub fn new() -> Result<Self> {
        let inner = unsafe { sys::av_frame_alloc() };
        if inner.is_null() {
            return Err(FFmpegError::new(-1));
        }
        Ok(Frame { inner })
    }

    pub fn as_mut_ptr(&mut self) -> *mut sys::AVFrame {
        self.inner
    }

    pub fn as_ptr(&self) -> *const sys::AVFrame {
        self.inner
    }

    pub fn is_key_frame(&self) -> bool {
        unsafe { (*self.inner).key_frame != 0 }
    }

    pub fn pts(&self) -> i64 {
        unsafe { (*self.inner).pts }
    }

    pub fn width(&self) -> i32 {
        unsafe { (*self.inner).width }
    }

    pub fn height(&self) -> i32 {
        unsafe { (*self.inner).height }
    }

    pub fn data(&self, plane: usize) -> &[u8] {
        unsafe {
            let data = (*self.inner).data[plane];
            let linesize = (*self.inner).linesize[plane] as usize;
            let height = self.height() as usize;
            slice::from_raw_parts(data, linesize * height)
        }
    }

    pub fn data_mut(&mut self, plane: usize) -> &mut [u8] {
        unsafe {
            let data = (*self.inner).data[plane];
            let linesize = (*self.inner).linesize[plane] as usize;
            let height = self.height() as usize;
            slice::from_raw_parts_mut(data, linesize * height)
        }
    }

    pub fn linesize(&self, plane: usize) -> i32 {
        unsafe { (*self.inner).linesize[plane] }
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe {
            sys::av_frame_free(&mut self.inner);
        }
    }
}
