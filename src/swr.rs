use crate::AVSampleFormat;
use crate::error::{Error, Result};
use crate::frame::Frame;
use libavcodec_sys as sys;
use std::ptr;

/// Specifies the resampling algorithm to use
#[derive(Debug, Clone, Copy)]
pub enum ResampleAlgorithm {
    /// Fast but lower quality resampling (linear interpolation)
    Linear,
    /// Balanced quality/speed resampling (cubic interpolation)
    Cubic,
    /// High quality resampling using sinc-based algorithm with Blackman window
    /// The quality parameter determines the size of the filter (higher = better but slower)
    Sinc {
        /// Quality level from 0 (lowest) to 10 (highest)
        quality: i32,
    },
}

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
        Self::get_context_with_algorithm(
            in_sample_rate,
            in_sample_fmt,
            in_channel_count,
            out_sample_rate,
            out_sample_fmt,
            out_channel_count,
            ResampleAlgorithm::Cubic, // default to cubic interpolation
        )
    }

    pub fn get_context_with_algorithm(
        in_sample_rate: usize,
        in_sample_fmt: AVSampleFormat,
        in_channel_count: usize,
        out_sample_rate: usize,
        out_sample_fmt: AVSampleFormat,
        out_channel_count: usize,
        algorithm: ResampleAlgorithm,
    ) -> Result<Self> {
        // Create default channel layouts based on channel counts
        let mut in_ch_layout = unsafe { std::mem::zeroed::<sys::AVChannelLayout>() };
        let mut out_ch_layout = unsafe { std::mem::zeroed::<sys::AVChannelLayout>() };

        // Initialize channel layouts with just channel counts
        in_ch_layout.order = sys::AVChannelOrder_AV_CHANNEL_ORDER_UNSPEC;
        in_ch_layout.nb_channels = in_channel_count as i32;

        out_ch_layout.order = sys::AVChannelOrder_AV_CHANNEL_ORDER_UNSPEC;
        out_ch_layout.nb_channels = out_channel_count as i32;

        let mut inner = ptr::null_mut();
        let ret = unsafe {
            sys::swr_alloc_set_opts2(
                &mut inner,
                &out_ch_layout,         // out_ch_layout
                out_sample_fmt as i32,  // out_sample_fmt
                out_sample_rate as i32, // out_sample_rate
                &in_ch_layout,          // in_ch_layout
                in_sample_fmt as i32,   // in_sample_fmt
                in_sample_rate as i32,  // in_sample_rate
                0,                      // log_offset
                ptr::null_mut(),        // log_ctx
            )
        };

        // Clean up channel layouts
        unsafe {
            sys::av_channel_layout_uninit(&mut in_ch_layout);
            sys::av_channel_layout_uninit(&mut out_ch_layout);
        }

        if ret < 0 {
            return Err(Error::new(ret));
        }

        let mut inner = unsafe { &mut *inner };

        // Configure resampling algorithm
        unsafe {
            match algorithm {
                ResampleAlgorithm::Linear => {
                    // Linear interpolation - fast but lower quality
                    sys::av_opt_set_int(
                        inner as *mut _ as *mut _,
                        "linear_interp\0".as_ptr() as *const i8,
                        1,
                        0,
                    );
                    sys::av_opt_set_int(
                        inner as *mut _ as *mut _,
                        "filter_type\0".as_ptr() as *const i8,
                        0, // 0 = cubic
                        0,
                    );
                }
                ResampleAlgorithm::Cubic => {
                    // Cubic interpolation - good balance of quality/speed
                    sys::av_opt_set_int(
                        inner as *mut _ as *mut _,
                        "linear_interp\0".as_ptr() as *const i8,
                        0,
                        0,
                    );
                    sys::av_opt_set_int(
                        inner as *mut _ as *mut _,
                        "filter_type\0".as_ptr() as *const i8,
                        0, // 0 = cubic
                        0,
                    );
                }
                ResampleAlgorithm::Sinc { quality } => {
                    // Sinc resampling with configurable quality
                    let quality = quality.clamp(0, 10);
                    sys::av_opt_set_int(
                        inner as *mut _ as *mut _,
                        "linear_interp\0".as_ptr() as *const i8,
                        0,
                        0,
                    );
                    sys::av_opt_set_int(
                        inner as *mut _ as *mut _,
                        "filter_type\0".as_ptr() as *const i8,
                        1, // 1 = sinc
                        0,
                    );
                    sys::av_opt_set_int(
                        inner as *mut _ as *mut _,
                        "filter_size\0".as_ptr() as *const i8,
                        (16 + quality * 8) as i64, // filter size based on quality
                        0,
                    );
                }
            }
        }

        // initialize the resampler
        let ret = unsafe { sys::swr_init(inner) };
        if ret < 0 {
            unsafe { sys::swr_free(&mut inner as *mut _ as *mut _) };
            return Err(Error::new(ret));
        }

        Ok(SwrContext { inner })
    }

    pub fn convert(&mut self, src: &Frame, dst: &mut Frame) -> Result<()> {
        let ret = unsafe { sys::swr_convert_frame(self.inner, dst.as_mut_ptr(), src.as_ptr()) };

        if ret < 0 {
            Err(Error::new(ret))
        } else {
            Ok(())
        }
    }

    pub fn get_delay(&self, base: i32) -> i64 {
        unsafe { sys::swr_get_delay(self.inner, base as i64) }
    }

    pub fn convert_frame(&mut self, src: Option<&Frame>, dst: &mut Frame) -> Result<()> {
        let src_ptr = src.map_or(std::ptr::null(), |f| f.as_ptr());
        let ret = unsafe { sys::swr_convert_frame(self.inner, dst.as_mut_ptr(), src_ptr) };

        if ret < 0 {
            Err(Error::new(ret))
        } else {
            Ok(())
        }
    }

    pub fn get_out_samples(&self, in_samples: i64) -> i64 {
        unsafe {
            sys::av_rescale_rnd(
                in_samples,
                self.get_out_rate(),
                self.get_in_rate(),
                sys::AVRounding_AV_ROUND_UP,
            )
        }
    }

    pub fn get_in_rate(&self) -> i64 {
        unsafe {
            let mut rate = 0;
            sys::av_opt_get_int(
                self.inner as *mut _ as *mut _,
                "in_sample_rate\0".as_ptr() as *const i8,
                0,
                &mut rate,
            );
            rate
        }
    }

    pub fn get_out_rate(&self) -> i64 {
        unsafe {
            let mut rate = 0;
            sys::av_opt_get_int(
                self.inner as *mut _ as *mut _,
                "out_sample_rate\0".as_ptr() as *const i8,
                0,
                &mut rate,
            );
            rate
        }
    }

    pub fn set_compensation(
        &mut self,
        sample_delta: i32,
        compensation_distance: i32,
    ) -> Result<()> {
        let ret =
            unsafe { sys::swr_set_compensation(self.inner, sample_delta, compensation_distance) };

        if ret < 0 {
            Err(Error::new(ret))
        } else {
            Ok(())
        }
    }

    pub fn drop_output(&mut self, count: i32) -> Result<()> {
        let ret = unsafe { sys::swr_drop_output(self.inner, count) };
        if ret < 0 {
            Err(Error::new(ret))
        } else {
            Ok(())
        }
    }

    pub fn inject_silence(&mut self, count: i32) -> Result<()> {
        let ret = unsafe { sys::swr_inject_silence(self.inner, count) };
        if ret < 0 {
            Err(Error::new(ret))
        } else {
            Ok(())
        }
    }
}

impl Drop for SwrContext {
    fn drop(&mut self) {
        unsafe {
            sys::swr_free(&mut self.inner);
        }
    }
}
