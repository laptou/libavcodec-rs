use crate::AVCodecId;
use crate::AVDiscard;
use crate::AVPixelFormat;
use crate::AVSampleFormat;
use crate::ChannelLayout;
use crate::Error;
use crate::Rational;
use crate::error::Result;
use crate::frame::Frame;
use crate::packet::Packet;
use libavcodec_sys as sys;
use num_traits::FromPrimitive;
use std::ptr;
use std::ptr::NonNull;

pub struct Codec {
    inner: *const sys::AVCodec,
}

unsafe impl Send for Codec {}

impl Codec {
    pub fn find_decoder(id: AVCodecId) -> Option<Self> {
        let inner = unsafe { sys::avcodec_find_decoder(id as _) };
        if inner.is_null() {
            None
        } else {
            Some(Codec { inner })
        }
    }

    pub fn find_encoder(id: AVCodecId) -> Option<Self> {
        let inner = unsafe { sys::avcodec_find_encoder(id as _) };
        if inner.is_null() {
            None
        } else {
            Some(Codec { inner })
        }
    }

    pub fn as_ptr(&self) -> *const sys::AVCodec {
        self.inner
    }
}

pub struct CodecContext {
    inner: NonNull<sys::AVCodecContext>,
}

unsafe impl Send for CodecContext {}

impl AsRef<sys::AVCodecContext> for CodecContext {
    fn as_ref(&self) -> &sys::AVCodecContext {
        unsafe { self.inner.as_ref() }
    }
}

impl AsMut<sys::AVCodecContext> for CodecContext {
    fn as_mut(&mut self) -> &mut sys::AVCodecContext {
        unsafe { self.inner.as_mut() }
    }
}

impl CodecContext {
    pub fn new(codec: &Codec) -> Result<Self> {
        let inner = unsafe { sys::avcodec_alloc_context3(codec.as_ptr()) };
        let inner = NonNull::new(inner).ok_or(Error::Alloc)?;

        Ok(CodecContext { inner })
    }

    pub fn as_mut_ptr(&mut self) -> *mut sys::AVCodecContext {
        self.inner.as_ptr()
    }

    pub fn as_ptr(&self) -> *const sys::AVCodecContext {
        self.inner.as_ptr()
    }

    pub fn width(&self) -> usize {
        self.as_ref().width as usize
    }

    pub fn height(&self) -> usize {
        self.as_ref().height as usize
    }

    /// The pixel format of the video.  
    pub fn pixel_format(&self) -> AVPixelFormat {
        AVPixelFormat::from_i32(self.as_ref().pix_fmt).unwrap()
    }

    /// The sample format of the audio.
    pub fn sample_format(&self) -> AVSampleFormat {
        AVSampleFormat::from_i32(self.as_ref().sample_fmt).unwrap()
    }

    /// The sample rate of the audio.
    pub fn sample_rate(&self) -> usize {
        self.as_ref().sample_rate as usize
    }

    /// The number of audio channels.
    pub fn channel_count(&self) -> usize {
        self.as_ref().ch_layout.nb_channels as usize
    }

    pub fn channel_layout(&self) -> ChannelLayout {
        ChannelLayout(self.as_ref().ch_layout)
    }

    /// The number of audio samples per channel in an audio frame.
    pub fn frame_size(&self) -> usize {
        self.as_ref().frame_size as usize
    }

    pub fn set_skip_frame(&mut self, value: AVDiscard) {
        self.as_mut().skip_frame = value as i32;
    }

    pub fn open(&mut self, codec: &Codec) -> Result<()> {
        let ret = unsafe { sys::avcodec_open2(self.as_mut(), codec.as_ptr(), ptr::null_mut()) };
        if ret < 0 {
            Err(Error::new(ret))
        } else {
            Ok(())
        }
    }

    pub fn send_packet(&mut self, packet: &Packet) -> Result<()> {
        let ret = unsafe { sys::avcodec_send_packet(self.as_mut(), packet.as_ref()) };
        if ret < 0 {
            Err(Error::new(ret))
        } else {
            Ok(())
        }
    }

    pub fn receive_frame(&mut self, frame: &mut Frame) -> Result<()> {
        let ret = unsafe { sys::avcodec_receive_frame(self.as_mut(), frame.as_mut_ptr()) };
        if ret < 0 {
            Err(Error::new(ret))
        } else {
            Ok(())
        }
    }

    pub fn time_base(&self) -> Rational {
        self.as_ref().time_base.into()
    }

    pub fn set_sample_rate(&mut self, sample_rate: i32) {
        self.as_mut().sample_rate = sample_rate;
    }

    pub fn set_sample_format(&mut self, format: AVSampleFormat) {
        self.as_mut().sample_fmt = format as i32;
    }

    pub fn set_channel_count(&mut self, count: i32) {
        unsafe {
            sys::av_channel_layout_default(&mut self.as_mut().ch_layout, count);
        }
    }

    pub fn set_time_base(&mut self, time_base: Rational) {
        self.as_mut().time_base = time_base.into();
    }

    pub fn send_frame(&mut self, frame: Option<&Frame>) -> Result<()> {
        let frame_ptr = frame.map_or(std::ptr::null(), |f| f.as_ptr());
        let ret = unsafe { sys::avcodec_send_frame(self.as_mut(), frame_ptr) };
        if ret < 0 {
            Err(Error::new(ret))
        } else {
            Ok(())
        }
    }

    pub fn receive_packet(&mut self, packet: &mut Packet) -> Result<()> {
        let ret = unsafe { sys::avcodec_receive_packet(self.as_mut(), packet.as_mut()) };
        if ret < 0 {
            Err(Error::new(ret))
        } else {
            Ok(())
        }
    }
}

impl Drop for CodecContext {
    fn drop(&mut self) {
        unsafe { sys::avcodec_free_context(&mut self.inner.as_ptr()) }
    }
}
