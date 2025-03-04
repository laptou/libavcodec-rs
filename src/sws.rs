use crate::AVPixelFormat;
use crate::error::{FFmpegError, Result};
use crate::frame::Frame;
use libavcodec_sys as sys;
use std::ptr;

pub struct SwsContext {
    inner: *mut sys::SwsContext,
}

unsafe impl Send for SwsContext {}

impl SwsContext {
    pub fn get_context(
        src_width: usize,
        src_height: usize,
        src_pix_fmt: AVPixelFormat,
        dst_width: usize,
        dst_height: usize,
        dst_pix_fmt: AVPixelFormat,
        flags: i32,
    ) -> Result<Self> {
        let inner = unsafe {
            sys::sws_getContext(
                src_width as i32,
                src_height as i32,
                src_pix_fmt as i32,
                dst_width as i32,
                dst_height as i32,
                dst_pix_fmt as i32,
                flags,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };

        if inner.is_null() {
            Err(FFmpegError::new(-1))
        } else {
            Ok(SwsContext { inner })
        }
    }

    pub fn copy(&mut self, src: &Frame, dst: &mut Frame) -> Result<()> {
        let ret = unsafe {
            sys::sws_scale(
                self.inner,
                src.data_ptrs().as_ptr() as _,
                src.data_line_sizes().as_ptr(),
                0,
                src.height(),
                dst.data_ptrs().as_ptr(),
                dst.data_line_sizes().as_ptr(),
            )
        };

        if ret < 0 {
            Err(FFmpegError::new(ret))
        } else {
            Ok(())
        }
    }
}

impl Drop for SwsContext {
    fn drop(&mut self) {
        unsafe {
            sys::sws_freeContext(self.inner);
        }
    }
}
