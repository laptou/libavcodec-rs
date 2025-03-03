use crate::error::{FFmpegError, Result};
use libavcodec_sys as sys;
use std::{ptr::NonNull, slice};

pub struct Frame {
    inner: NonNull<sys::AVFrame>,
    // store rust-allocated buffer if we create one
    buffer: Option<Vec<u8>>,
}

impl Frame {
    pub fn new() -> Result<Self> {
        let inner = unsafe { sys::av_frame_alloc() };
        let inner = NonNull::new(inner).ok_or(FFmpegError::new(-1))?;
        Ok(Frame {
            inner,
            buffer: None,
        })
    }

    /// Allocate a new buffer for this frame with the given parameters and set up the frame's
    /// data pointers and linesize information.
    pub fn allocate_buffer(
        &mut self,
        width: i32,
        height: i32,
        pix_fmt: sys::AVPixelFormat,
        align: i32,
    ) -> Result<()> {
        // get required buffer size
        let size = unsafe { sys::av_image_get_buffer_size(pix_fmt, width, height, align) };
        if size < 0 {
            return Err(FFmpegError::new(size));
        }

        // allocate buffer
        let mut buffer = vec![0u8; size as usize];

        unsafe {
            // setup frame parameters
            self.inner_mut().width = width;
            self.inner_mut().height = height;
            self.inner_mut().format = pix_fmt as i32;

            // setup data pointers and linesize
            let ret = sys::av_image_fill_arrays(
                self.inner_mut().data.as_mut_ptr(),
                self.inner_mut().linesize.as_mut_ptr(),
                buffer.as_mut_ptr(),
                pix_fmt,
                width,
                height,
                align,
            );
            if ret < 0 {
                return Err(FFmpegError::new(ret));
            }
        }

        // store buffer so it lives as long as the frame
        self.buffer = Some(buffer);
        Ok(())
    }

    /// Alternative allocation method that uses av_image_alloc directly.
    /// This lets ffmpeg handle the allocation but we still need to store the buffer
    /// to ensure proper cleanup.
    pub fn allocate_buffer_ffmpeg(
        &mut self,
        width: i32,
        height: i32,
        pix_fmt: sys::AVPixelFormat,
        align: i32,
    ) -> Result<()> {
        unsafe {
            // setup frame parameters
            self.inner_mut().width = width;
            self.inner_mut().height = height;
            self.inner_mut().format = pix_fmt as i32;

            let ret = sys::av_image_alloc(
                self.inner_mut().data.as_mut_ptr(),
                self.inner_mut().linesize.as_mut_ptr(),
                width,
                height,
                pix_fmt,
                align,
            );

            if ret < 0 {
                return Err(FFmpegError::new(ret));
            }
        }
        Ok(())
    }

    pub fn as_mut_ptr(&mut self) -> *mut sys::AVFrame {
        self.inner.as_ptr()
    }

    pub fn as_ptr(&self) -> *const sys::AVFrame {
        self.inner.as_ptr()
    }

    fn inner(&self) -> &sys::AVFrame {
        unsafe { self.inner.as_ref() }
    }

    fn inner_mut(&mut self) -> &mut sys::AVFrame {
        unsafe { self.inner.as_mut() }
    }

    pub fn is_key_frame(&self) -> bool {
        self.inner().key_frame != 0
    }

    pub fn pts(&self) -> i64 {
        self.inner().pts
    }

    pub fn width(&self) -> i32 {
        self.inner().width
    }

    pub fn height(&self) -> i32 {
        self.inner().height
    }

    pub fn data(&self, plane: usize) -> Option<&[u8]> {
        unsafe {
            let data = self.data_ptr(plane);
            let linesize = self.data_line_size(plane);

            if data.is_null() || linesize <= 0 {
                // can't construct a slice if linesize is negative, this means
                // we're supposed to iterate in reverse
                None
            } else {
                let linesize = linesize as usize;
                let height = self.height() as usize;
                Some(slice::from_raw_parts(data, linesize * height))
            }
        }
    }

    pub fn data_mut(&mut self, plane: usize) -> Option<&mut [u8]> {
        unsafe {
            let data = self.data_ptr_mut(plane);
            let linesize = self.data_line_size(plane);

            if data.is_null() || linesize <= 0 {
                // can't construct a slice if linesize is negative, this means
                // we're supposed to iterate in reverse
                None
            } else {
                let linesize = linesize as usize;
                let height = self.height() as usize;
                Some(slice::from_raw_parts_mut(data, linesize * height))
            }
        }
    }

    pub unsafe fn data_ptrs(&self) -> &[*mut u8] {
        &self.inner().data
    }

    pub unsafe fn data_ptr(&self, plane: usize) -> *const u8 {
        self.inner().data[plane]
    }

    pub unsafe fn data_ptr_mut(&mut self, plane: usize) -> *mut u8 {
        self.inner_mut().data[plane]
    }

    pub fn data_line_size(&self, plane: usize) -> isize {
        self.inner().linesize[plane] as isize
    }

    pub fn data_line_sizes(&self) -> &[i32] {
        &self.inner().linesize
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        // buffer will be dropped automatically when self.buffer is dropped
        unsafe {
            sys::av_frame_free(&mut self.inner.as_ptr());
        }
    }
}
