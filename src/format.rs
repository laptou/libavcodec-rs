use crate::error::{Error, Result};
use crate::io_context::{IoContext, ReadFn, SeekFn};
use crate::packet::Packet;
use crate::{AVError, IoContextParams, Stream};
use libavcodec_sys as sys;
use std::ffi::CString;
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::ptr::{self, NonNull};
use std::time::Duration;

pub struct FormatContext<D = ()> {
    inner: NonNull<sys::AVFormatContext>,
    // Keep IoContext alive as long as this FormatContext is alive
    // This is needed because the IoContext has callbacks that need to remain valid
    io_context: Option<IoContext<D>>,
}

unsafe impl<D> Send for FormatContext<D> {}

impl<D> AsRef<sys::AVFormatContext> for FormatContext<D> {
    fn as_ref(&self) -> &sys::AVFormatContext {
        unsafe { self.inner.as_ref() }
    }
}

impl<D> AsMut<sys::AVFormatContext> for FormatContext<D> {
    fn as_mut(&mut self) -> &mut sys::AVFormatContext {
        unsafe { self.inner.as_mut() }
    }
}

impl<D> FormatContext<D> {
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
            return Err(Error::new(ret));
        }

        let ret = unsafe { sys::avformat_find_stream_info(inner, ptr::null_mut()) };

        if ret < 0 {
            unsafe { sys::avformat_close_input(&mut inner) };
            return Err(Error::new(ret));
        }

        let inner = NonNull::new(inner).unwrap();

        Ok(FormatContext {
            inner,
            io_context: None,
        })
    }

    pub unsafe fn from_raw(ptr: NonNull<sys::AVFormatContext>) -> Self {
        Self {
            inner: ptr,
            io_context: None,
        }
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

    pub fn stream_count(&self) -> usize {
        self.as_ref().nb_streams as usize
    }

    pub fn duration(&self) -> Option<Duration> {
        let duration = self.as_ref().duration;

        if duration < 0 {
            None
        } else {
            Some(Duration::from_micros(duration as u64))
        }
    }

    pub fn read_packet(&mut self, packet: &mut Packet) -> Result<bool> {
        let ret = unsafe { sys::av_read_frame(self.as_mut(), packet.as_mut()) };

        if ret < 0 {
            if ret == AVError::Eof as i32 {
                Ok(false)
            } else {
                Err(Error::new(ret))
            }
        } else {
            Ok(true)
        }
    }

    pub fn output<P: AsRef<Path>>(path: P, format: Option<&str>) -> Result<Self> {
        let path_str = path.as_ref().to_str().ok_or(Error::Utf8)?;
        let path_cstr = CString::new(path_str).map_err(|_| Error::NulByte)?;
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
            return Err(Error::new(ret));
        }

        let ctx = NonNull::new(ctx).ok_or(Error::Alloc)?;

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
                return Err(Error::new(ret));
            }
        }

        Ok(FormatContext {
            inner: ctx,
            io_context: None,
        })
    }

    pub fn write_header(&mut self) -> Result<()> {
        let ret = unsafe { sys::avformat_write_header(self.as_mut(), std::ptr::null_mut()) };
        if ret < 0 {
            Err(Error::new(ret))
        } else {
            Ok(())
        }
    }

    pub fn write_frame(&mut self, packet: &mut Packet) -> Result<()> {
        let ret = unsafe { sys::av_write_frame(self.as_mut(), packet.as_mut()) };
        if ret < 0 {
            Err(Error::new(ret))
        } else {
            Ok(())
        }
    }

    pub fn write_frame_interleaved(&mut self, packet: &mut Packet) -> Result<()> {
        let ret = unsafe { sys::av_interleaved_write_frame(self.as_mut(), packet.as_mut()) };
        if ret < 0 {
            Err(Error::new(ret))
        } else {
            Ok(())
        }
    }

    pub fn write_trailer(&mut self) -> Result<()> {
        let ret = unsafe { sys::av_write_trailer(self.as_mut()) };
        if ret < 0 {
            Err(Error::new(ret))
        } else {
            Ok(())
        }
    }

    pub fn new_stream(&mut self) -> Result<Stream> {
        let stream = unsafe { sys::avformat_new_stream(self.as_mut(), ptr::null()) };
        let stream = NonNull::new(stream).ok_or(Error::Alloc)?;

        Ok(Stream { inner: stream })
    }

    pub fn from_io(io: Io<D>) -> Result<Self> {
        match io {
            Io::File(path) => {
                // existing file-based approach
                Self::open(&path)
            }
            Io::Custom {
                data,
                params,
                file_name,
            } => Self::with_io_context(IoContext::new(data, params)?, file_name.as_deref()),
        }
    }

    /// Creates a new FormatContext for an input source using a custom IoContext
    ///
    /// # Parameters
    /// * `io_context` - The custom IoContext to use for I/O operations
    /// * `file_name` - The name of the file to open
    ///
    /// This method creates a FormatContext that reads from the provided IoContext,
    /// which can be used to read from arbitrary sources with custom read/write/seek
    /// callbacks.
    pub fn with_io_context(io_context: IoContext<D>, file_name: Option<&Path>) -> Result<Self> {
        let mut ctx = Self::alloc()?;
        ctx.set_io_context(io_context);
        ctx.open_input(file_name)
    }

    /// Creates an uninitialized FormatContext
    ///
    /// This is a low-level function typically used internally.
    /// You should prefer the `open`, `output`, or `with_io_context` methods instead.
    pub fn alloc() -> Result<Self> {
        unsafe {
            let ctx = sys::avformat_alloc_context();
            if ctx.is_null() {
                return Err(Error::Alloc);
            }

            let ptr = NonNull::new(ctx).ok_or(Error::Alloc)?;
            Ok(FormatContext {
                inner: ptr,
                io_context: None,
            })
        }
    }

    /// Set the custom IoContext for this FormatContext
    ///
    /// This method allows setting a custom IoContext for I/O operations.
    /// This is typically used before calling `open_input`.
    pub fn set_io_context(&mut self, mut io_context: IoContext<D>) {
        unsafe {
            (*self.inner.as_ptr()).pb = io_context.as_mut_ptr();
        }

        self.io_context = Some(io_context);
    }

    /// Open input without an explicit path, for use with custom IoContext
    ///
    /// This method is used to open an input after setting up a format context
    /// with a custom IoContext.
    pub fn open_input(self, file_name: Option<&Path>) -> Result<Self> {
        unsafe {
            let file_name_cstr = match file_name {
                Some(path) => {
                    let path_str = path.to_str().ok_or(Error::Utf8)?;
                    let path_cstr = CString::new(path_str).unwrap();
                    Some(path_cstr)
                }
                None => None,
            };

            // open input - we need to use a mutable pointer for avformat_open_input
            let mut ctx_ptr = self.inner.as_ptr();
            let ret = sys::avformat_open_input(
                &mut ctx_ptr,
                file_name_cstr.map_or(ptr::null_mut(), |s| s.as_ptr()),
                ptr::null(),
                ptr::null_mut(),
            );

            if ret < 0 {
                // this method consumes self b/c if avformat_open_input fails,
                // it frees the context!
                return Err(Error::new(ret));
            }

            let ret = sys::avformat_find_stream_info(ctx_ptr, ptr::null_mut());

            if ret < 0 {
                sys::avformat_close_input(&mut ctx_ptr);
                return Err(Error::new(ret));
            }

            Ok(self)
        }
    }
}

impl<D> Drop for FormatContext<D> {
    fn drop(&mut self) {
        unsafe {
            // sys::avformat_close_input(&mut self.inner);

            // if let Some(fmt) = self.as_ref().oformat.as_ref() {
            //     if (fmt.flags & sys::AVFMT_NOFILE as i32) == 0 {
            //         let mut pb = self.as_mut().pb;
            //         if !pb.is_null() {
            //             // Only close pb if we don't have an _io_context
            //             // If we have an _io_context, its Drop impl will handle this
            //             if self.io_context.is_none() {
            //                 sys::avio_closep(&mut pb);
            //             }
            //         }
            //     }
            // }

            sys::avformat_free_context(self.as_mut_ptr());
        }
    }
}

pub enum Io<D = ()> {
    /// File path for direct reading
    File(PathBuf),
    /// I/O callbacks for custom I/O
    Custom {
        data: D,
        params: IoContextParams<D>,
        /// Optional file name, used to infer file format
        file_name: Option<PathBuf>,
    },
}

// Adaptor functions to create callbacks from common types
impl<D> Io<D> {
    /// Create an AudioSource from a Read implementation
    pub fn from_reader(reader: D, file_name: Option<PathBuf>) -> Self
    where
        D: Read + 'static,
    {
        let read_fn = Box::new(|reader: &mut D, buf: &mut [u8]| reader.read(buf)) as ReadFn<D>;

        Self::Custom {
            data: reader,
            params: IoContextParams::Read {
                read_fn,
                seek_fn: None,
                buffer_size: 32768,
            },
            file_name,
        }
    }

    /// Create an AudioSource from a Read + Seek implementation
    pub fn from_seekable(reader: D, file_name: Option<PathBuf>) -> Self
    where
        D: Read + Seek + 'static,
    {
        let read_fn = Box::new(|reader: &mut D, buf: &mut [u8]| reader.read(buf)) as ReadFn<D>;
        let seek_fn = Box::new(|reader: &mut D, pos: SeekFrom| reader.seek(pos)) as SeekFn<D>;

        Self::Custom {
            data: reader,
            params: IoContextParams::Read {
                read_fn,
                seek_fn: Some(seek_fn),
                buffer_size: 32768,
            },
            file_name,
        }
    }
}

impl Io<Cursor<Vec<u8>>> {
    /// Create an AudioSource from raw bytes
    pub fn from_bytes(bytes: Vec<u8>, file_name: Option<PathBuf>) -> Self {
        let cursor = std::io::Cursor::new(bytes);
        Io::from_seekable(cursor, file_name)
    }
}

impl Io<File> {
    /// Create an AudioSource from a file
    pub fn from_file(path: &Path) -> Result<Self> {
        Ok(Io::File(path.to_path_buf()))
        // let file = std::fs::File::open(path)?;
        // Ok(Self::from_seekable(file, Some(path.to_path_buf())))
    }
}
