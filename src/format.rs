use crate::Stream;
use crate::error::{FFmpegError, Result};
use crate::packet::Packet;
use libavcodec_sys as sys;
use std::ffi::CString;
use std::path::Path;
use std::ptr::{self, NonNull};

pub struct FormatContext {
    inner: NonNull<sys::AVFormatContext>,
}

unsafe impl Send for FormatContext {}

impl AsRef<sys::AVFormatContext> for FormatContext {
    fn as_ref(&self) -> &sys::AVFormatContext {
        unsafe { self.inner.as_ref() }
    }
}

impl AsMut<sys::AVFormatContext> for FormatContext {
    fn as_mut(&mut self) -> &mut sys::AVFormatContext {
        unsafe { self.inner.as_mut() }
    }
}

impl FormatContext {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_str = path.as_ref().to_string_lossy();
        let path_cstr = CString::new(path_str.as_bytes()).unwrap();
        let mut inner = ptr::null_mut();

        let ret = unsafe {
            sys::avformat_open_input(
                &mut inner,
                path_cstr.as_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };

        if ret < 0 {
            return Err(FFmpegError::new(ret));
        }

        let ret = unsafe { sys::avformat_find_stream_info(inner, ptr::null_mut()) };

        if ret < 0 {
            unsafe { sys::avformat_close_input(&mut inner) };
            return Err(FFmpegError::new(ret));
        }

        let inner = NonNull::new(inner).unwrap();

        Ok(FormatContext { inner })
    }

    pub fn as_mut_ptr(&mut self) -> *mut sys::AVFormatContext {
        self.inner.as_ptr()
    }

    pub fn as_ptr(&self) -> *const sys::AVFormatContext {
        self.inner.as_ptr()
    }

    pub fn streams(&self) -> impl Iterator<Item = Stream> {
        let nb_streams = self.as_ref().nb_streams;

        let streams =
            unsafe { std::slice::from_raw_parts(self.as_ref().streams, nb_streams as usize) };

        streams.iter().map(|&ptr| Stream {
            inner: NonNull::new(ptr).unwrap(),
        })
    }

    pub fn read_packet(&mut self, packet: &mut Packet) -> Result<bool> {
        let ret = unsafe { sys::av_read_frame(self.inner.as_ptr(), packet.as_mut_ptr()) };

        if ret < 0 {
            if ret == sys::AVErrorEof {
                Ok(false)
            } else {
                Err(FFmpegError::new(ret))
            }
        } else {
            Ok(true)
        }
    }

    pub fn output<P: AsRef<Path>>(path: P, format: Option<&str>) -> Result<Self> {
        let path_str = path.as_ref().to_str().ok_or(FFmpegError::new(-1))?;
        let path_cstr = CString::new(path_str).map_err(|_| FFmpegError::new(-1))?;
        let format_cstr = format.map(|f| CString::new(f).unwrap());

        let mut ctx = std::ptr::null_mut();
        let ret = unsafe {
            sys::avformat_alloc_output_context2(
                &mut ctx,
                std::ptr::null(),
                format_cstr
                    .as_ref()
                    .map(|s| s.as_ptr())
                    .unwrap_or(std::ptr::null()),
                path_cstr.as_ptr(),
            )
        };

        if ret < 0 {
            return Err(FFmpegError::new(ret));
        }

        let ctx = NonNull::new(ctx).ok_or(FFmpegError::new(-1))?;

        // Open output file if needed
        if unsafe { (*ctx.as_ptr()).oformat.as_ref() }
            .map(|f| (f.flags & sys::AVFMT_NOFILE as i32) == 0)
            .unwrap_or(false)
        {
            let ret = unsafe {
                sys::avio_open(
                    &mut (*ctx.as_ptr()).pb,
                    path_cstr.as_ptr(),
                    sys::AVIO_FLAG_WRITE as i32,
                )
            };
            if ret < 0 {
                unsafe { sys::avformat_free_context(ctx.as_ptr()) };
                return Err(FFmpegError::new(ret));
            }
        }

        Ok(FormatContext { inner: ctx })
    }

    pub fn write_header(&mut self) -> Result<()> {
        let ret = unsafe { sys::avformat_write_header(self.inner.as_ptr(), std::ptr::null_mut()) };
        if ret < 0 {
            Err(FFmpegError::new(ret))
        } else {
            Ok(())
        }
    }

    pub fn write_frame(&mut self, packet: &mut Packet) -> Result<()> {
        let ret = unsafe { sys::av_write_frame(self.inner.as_ptr(), packet.as_mut_ptr()) };
        if ret < 0 {
            Err(FFmpegError::new(ret))
        } else {
            Ok(())
        }
    }

    pub fn write_frame_interleaved(&mut self, packet: &mut Packet) -> Result<()> {
        let ret =
            unsafe { sys::av_interleaved_write_frame(self.inner.as_ptr(), packet.as_mut_ptr()) };
        if ret < 0 {
            Err(FFmpegError::new(ret))
        } else {
            Ok(())
        }
    }

    pub fn write_trailer(&mut self) -> Result<()> {
        let ret = unsafe { sys::av_write_trailer(self.inner.as_ptr()) };
        if ret < 0 {
            Err(FFmpegError::new(ret))
        } else {
            Ok(())
        }
    }

    pub fn new_stream(&mut self) -> Result<Stream> {
        let stream = unsafe { sys::avformat_new_stream(self.inner.as_ptr(), ptr::null()) };
        let stream = NonNull::new(stream).ok_or(FFmpegError::new(-1))?;

        Ok(Stream { inner: stream })
    }
}

impl Drop for FormatContext {
    fn drop(&mut self) {
        unsafe {
            // sys::avformat_close_input(&mut self.inner);

            if let Some(fmt) = self.as_ref().oformat.as_ref() {
                if (fmt.flags & sys::AVFMT_NOFILE as i32) == 0 {
                    let mut pb = self.as_mut().pb;
                    if !pb.is_null() {
                        sys::avio_closep(&mut pb);
                    }
                }
            }

            sys::avformat_free_context(self.as_mut_ptr());
        }
    }
}
