use crate::error::{FFmpegError, Result};
use crate::frame::Frame;
use libavcodec_sys as sys;
use std::ptr;

pub struct SwsContext {
    inner: *mut sys::SwsContext,
}

impl SwsContext {
    pub fn get_context(
        src_width: i32,
        src_height: i32,
        src_pix_fmt: i32,
        dst_width: i32,
        dst_height: i32,
        dst_pix_fmt: i32,
        flags: i32,
    ) -> Result<Self> {
        let inner = unsafe {
            sys::sws_getContext(
                src_width,
                src_height,
                src_pix_fmt,
                dst_width,
                dst_height,
                dst_pix_fmt,
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

    pub fn scale(&mut self, src: &Frame, dst: &mut Frame) -> Result<()> {
        let ret = unsafe {
            sys::sws_scale(
                self.inner,
                src.as_ptr().cast(),
                (*src.as_ptr()).linesize.as_ptr(),
                0,
                (*src.as_ptr()).height,
                dst.as_mut_ptr().cast(),
                (*dst.as_mut_ptr()).linesize.as_ptr(),
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
