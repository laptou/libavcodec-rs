use crate::AVSampleFormat;
use crate::error::{FFmpegError, Result};
use crate::frame::Frame;
use libavcodec_sys as sys;
use std::ptr;

pub struct SwrContext {
    inner: *mut sys::SwrContext,
}

unsafe impl Send for SwrContext {}

impl SwrContext {
    pub fn get_context(
        in_sample_rate: usize,
        in_sample_fmt: AVSampleFormat,
        in_channel_count: usize,
        out_sample_rate: usize,
        out_sample_fmt: AVSampleFormat,
        out_channel_count: usize,
    ) -> Result<Self> {
        // Create default channel layouts based on channel counts
        let mut in_ch_layout = unsafe { std::mem::zeroed::<sys::AVChannelLayout>() };
        let mut out_ch_layout = unsafe { std::mem::zeroed::<sys::AVChannelLayout>() };
        
        unsafe {
            // Initialize channel layouts with just channel counts
            in_ch_layout.order = sys::AVChannelOrder_AV_CHANNEL_ORDER_UNSPEC;
            in_ch_layout.nb_channels = in_channel_count as i32;
            
            out_ch_layout.order = sys::AVChannelOrder_AV_CHANNEL_ORDER_UNSPEC;
            out_ch_layout.nb_channels = out_channel_count as i32;
        }

        let mut inner = ptr::null_mut();
        let ret = unsafe {
            sys::swr_alloc_set_opts2(
                &mut inner,
                &out_ch_layout,           // out_ch_layout
                out_sample_fmt as i32,    // out_sample_fmt
                out_sample_rate as i32,   // out_sample_rate
                &in_ch_layout,            // in_ch_layout
                in_sample_fmt as i32,     // in_sample_fmt
                in_sample_rate as i32,    // in_sample_rate
                0,                        // log_offset
                ptr::null_mut(),          // log_ctx
            )
        };

        // Clean up channel layouts
        unsafe {
            sys::av_channel_layout_uninit(&mut in_ch_layout);
            sys::av_channel_layout_uninit(&mut out_ch_layout);
        }

        if ret < 0 {
            return Err(FFmpegError::new(ret));
        }

        // initialize the resampler
        let ret = unsafe { sys::swr_init(inner) };
        if ret < 0 {
            unsafe { sys::swr_free(&mut inner) };
            return Err(FFmpegError::new(ret));
        }

        Ok(SwrContext { inner })
    }

    pub fn convert(&mut self, src: &Frame, dst: &mut Frame) -> Result<usize> {
        let ret = unsafe {
            sys::swr_convert_frame(
                self.inner,
                dst.as_mut_ptr(),
                src.as_ptr(),
            )
        };

        if ret < 0 {
            Err(FFmpegError::new(ret))
        } else {
            // Return number of samples output per channel
            Ok(dst.inner().nb_samples as usize)
        }
    }

    pub fn get_delay(&self, base: i32) -> i64 {
        unsafe { sys::swr_get_delay(self.inner, base as i64) }
    }
}

impl Drop for SwrContext {
    fn drop(&mut self) {
        unsafe {
            sys::swr_free(&mut self.inner);
        }
    }
} 
