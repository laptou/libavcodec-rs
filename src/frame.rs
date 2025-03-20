use crate::{error::Result, AVPixelFormat, AVSampleFormat, ChannelLayout, Error};
use libavcodec_sys as sys;
use std::{ptr::NonNull, slice};

pub struct Frame {
    inner: NonNull<sys::AVFrame>,
    // store rust-allocated buffer if we create one. AVFrame tracks buffer with
    // buf field, so as long as we don't set it, we shouldn't get double-free
    buffer: Option<Vec<u8>>,
}

unsafe impl Send for Frame {}

impl Frame {
    pub fn new() -> Result<Self> {
        let inner = unsafe { sys::av_frame_alloc() };
        let inner = NonNull::new(inner).ok_or(Error::Alloc)?;
        Ok(Frame {
            inner,
            buffer: None,
        })
    }

    /// Allocate a new buffer for this frame with the given parameters and set
    /// up the frame's data pointers and linesize information.
    pub fn allocate_image_buffer(
        &mut self,
        width: usize,
        height: usize,
        pix_fmt: AVPixelFormat,
        align: usize,
    ) -> Result<()> {
        let width = width as i32;
        let height = height as i32;
        let pix_fmt = pix_fmt as i32;
        let align = align as i32;

        // get required buffer size
        let size = unsafe { sys::av_image_get_buffer_size(pix_fmt, width, height, align) };
        if size < 0 {
            return Err(Error::new(size));
        }

        // allocate buffer
        let mut buffer = vec![0u8; size as usize];

        unsafe {
            // setup frame parameters
            self.inner_mut().width = width;
            self.inner_mut().height = height;
            self.inner_mut().format = pix_fmt;

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
                return Err(Error::new(ret));
            }
        }

        // store buffer so it lives as long as the frame
        self.buffer = Some(buffer);
        Ok(())
    }

    /// Alternative allocation method that uses av_image_alloc directly.
    /// This lets ffmpeg handle the allocation but we still need to store the buffer
    /// to ensure proper cleanup.
    pub fn allocate_image_buffer_av(
        &mut self,
        width: usize,
        height: usize,
        pix_fmt: AVPixelFormat,
        align: usize,
    ) -> Result<()> {
        let width = width as i32;
        let height = height as i32;
        let pix_fmt = pix_fmt as i32;
        let align = align as i32;

        unsafe {
            // setup frame parameters
            self.inner_mut().width = width;
            self.inner_mut().height = height;
            self.inner_mut().format = pix_fmt;

            let ret = sys::av_image_alloc(
                self.inner_mut().data.as_mut_ptr(),
                self.inner_mut().linesize.as_mut_ptr(),
                width,
                height,
                pix_fmt,
                align,
            );

            if ret < 0 {
                return Err(Error::new(ret));
            }
        }
        Ok(())
    }

    pub fn allocate_audio_buffer(
        &mut self,
        channel_layout: ChannelLayout,
        sample_rate: usize,
        sample_count: usize,
        sample_fmt: AVSampleFormat,
    ) -> Result<()> {
        let sample_rate = sample_rate as i32;
        let sample_fmt = sample_fmt as i32;

        unsafe {
            // setup frame parameters
            self.inner_mut().nb_samples = sample_count as i32;
            self.inner_mut().format = sample_fmt as i32;
            self.inner_mut().sample_rate = sample_rate;
            self.inner_mut().ch_layout = channel_layout.0;

            let mut linesize = 0;
            let size = sys::av_samples_get_buffer_size(
                &mut linesize,
                channel_layout.count() as i32,
                sample_count as i32,
                sample_fmt as i32,
                64, // Increased alignment
            );

            if size < 0 {
                return Err(Error::new(size));
            }

            let ret = sys::av_frame_get_buffer(self.inner_mut(), 64);
            if ret < 0 {
                return Err(Error::new(ret));
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

    pub fn inner(&self) -> &sys::AVFrame {
        unsafe { self.inner.as_ref() }
    }

    pub fn inner_mut(&mut self) -> &mut sys::AVFrame {
        unsafe { self.inner.as_mut() }
    }

    pub fn picture_data(&self, plane: usize) -> Option<&[u8]> {
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

    pub fn picture_data_mut(&mut self, plane: usize) -> Option<&mut [u8]> {
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

    

    pub fn audio_data(&self, plane: usize) -> Option<&[u8]> {
        unsafe {
            let data = self.data_ptr(plane);
            let linesize = self.data_line_size(plane);

            if data.is_null() || linesize <= 0 {
                // can't construct a slice if linesize is negative, this means
                // we're supposed to iterate in reverse
                None
            } else {
                let linesize = linesize as usize;
                let channels = self.inner().ch_layout.nb_channels as usize;
                Some(slice::from_raw_parts(data, linesize * channels))
            }
        }
    }

    pub fn audio_data_mut(&mut self, plane: usize) -> Option<&mut [u8]> {
        unsafe {
            let data = self.data_ptr_mut(plane);
            let linesize = self.data_line_size(plane);

            if data.is_null() || linesize <= 0 {
                // can't construct a slice if linesize is negative, this means
                // we're supposed to iterate in reverse
                None
            } else {
                let linesize = linesize as usize;
                let channels = self.inner().ch_layout.nb_channels as usize;
                Some(slice::from_raw_parts_mut(data, linesize * channels))
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

    pub fn is_key_frame(&self) -> bool {
        self.inner().key_frame != 0
    }

    pub fn pts(&self) -> i64 {
        self.inner().pts
    }

    pub fn pkt_dts(&self) -> i64 {
        self.inner().pkt_dts
    }

    pub fn best_effort_timestamp(&self) -> i64 {
        self.inner().best_effort_timestamp
    }

    pub fn width(&self) -> i32 {
        self.inner().width
    }

    pub fn height(&self) -> i32 {
        self.inner().height
    }

    pub fn sample_rate(&self) -> i32 {
        self.inner().sample_rate
    }

    pub fn sample_count(&self) -> i32 {
        self.inner().nb_samples
    }

    pub fn channel_count(&self) -> i32 {
        self.inner().ch_layout.nb_channels
    }

    pub fn channel_layout(&self) -> ChannelLayout {
        ChannelLayout(self.inner().ch_layout)
    }

    pub fn set_pts(&mut self, pts: i64) {
        self.inner_mut().pts = pts;
    }

    pub fn set_pkt_dts(&mut self, dts: i64) {
        self.inner_mut().pkt_dts = dts;
    }

    pub fn set_best_effort_timestamp(&mut self, timestamp: i64) {
        self.inner_mut().best_effort_timestamp = timestamp;
    }

    pub fn set_sample_rate(&mut self, rate: i32) {
        self.inner_mut().sample_rate = rate;
    }

    pub fn set_channel_layout(&mut self, layout: ChannelLayout) {
        self.inner_mut().ch_layout = layout.0;
    }

    pub fn set_format(&mut self, format: i32) {
        self.inner_mut().format = format;
    }

    pub fn set_nb_samples(&mut self, nb_samples: i32) {
        self.inner_mut().nb_samples = nb_samples;
    }

    pub fn make_writable(&mut self) -> Result<()> {
        let ret = unsafe { sys::av_frame_make_writable(self.inner_mut()) };
        if ret < 0 {
            Err(Error::new(ret))
        } else {
            Ok(())
        }
    }

    pub fn copy_props(&mut self, src: &Frame) -> Result<()> {
        let ret = unsafe { sys::av_frame_copy_props(self.inner_mut(), src.as_ptr()) };
        if ret < 0 {
            Err(Error::new(ret))
        } else {
            Ok(())
        }
    }

    pub fn samples_linesize(&self, align: i32) -> Result<i32> {
        let mut linesize = 0;
        let ret = unsafe {
            sys::av_samples_get_buffer_size(
                &mut linesize,
                self.channel_count(),
                self.sample_count(),
                self.format() as i32,
                align,
            )
        };
        if ret < 0 {
            Err(Error::new(ret))
        } else {
            Ok(linesize)
        }
    }

    pub fn format(&self) -> i32 {
        self.inner().format
    }

    pub fn unref(&mut self) {
        unsafe {
            sys::av_frame_unref(self.inner.as_ptr());
        }
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
