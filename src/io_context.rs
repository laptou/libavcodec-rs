use std::ptr::NonNull;
use std::{io::SeekFrom, pin::Pin};

use crate::{AVError, sys};

/// Function type for read operations
pub type ReadFn<D> = Box<dyn FnMut(&mut D, &mut [u8]) -> Result<usize, std::io::Error>>;

/// Function type for write operations
pub type WriteFn<D> = Box<dyn FnMut(&mut D, &[u8]) -> Result<usize, std::io::Error>>;

/// Function type for seek operations
pub type SeekFn<D> = Box<dyn FnMut(&mut D, SeekFrom) -> Result<u64, std::io::Error>>;

/// wrapper for custom AVIOContext to read from arbitrary sources using callbacks
pub struct IoContext<D = ()> {
    // user-provided data
    data: D,

    // user-provided callbacks
    read_fn: Option<ReadFn<D>>,
    write_fn: Option<WriteFn<D>>,
    seek_fn: Option<SeekFn<D>>,

    // the actual avio context
    inner: NonNull<sys::AVIOContext>,
}

unsafe impl Send for IoContext {}

impl AsRef<sys::AVIOContext> for IoContext {
    fn as_ref(&self) -> &sys::AVIOContext {
        unsafe { self.inner.as_ref() }
    }
}

impl AsMut<sys::AVIOContext> for IoContext {
    fn as_mut(&mut self) -> &mut sys::AVIOContext {
        unsafe { self.inner.as_mut() }
    }
}

pub enum IoContextParams<D> {
    Read {
        read_fn: ReadFn<D>,
        seek_fn: Option<SeekFn<D>>,
        buffer_size: usize,
    },
    Write {
        write_fn: WriteFn<D>,
        seek_fn: Option<SeekFn<D>>,
        buffer_size: usize,
    },
    ReadWrite {
        read_fn: ReadFn<D>,
        write_fn: WriteFn<D>,
        seek_fn: Option<SeekFn<D>>,
        buffer_size: usize,
    },
}

impl<D> IoContext<D> {
    /// Create a new CustomIOContext with the provided callbacks
    ///
    /// # Parameters
    /// * `read_fn` - Callback for reading data
    /// * `write_fn` - Callback for writing data
    /// * `seek_fn` - Optional callback for seeking
    /// * `buffer_size` - Size of the buffer to use for IO operations
    ///
    /// The IO mode is inferred from the provided callbacks:
    /// - If only `read_fn` is provided: Read-only mode
    /// - If only `write_fn` is provided: Write-only mode
    /// - If both are provided: Read-write mode
    pub fn new(data: D, params: IoContextParams<D>) -> crate::Result<Pin<Box<Self>>> {
        let (read_fn, write_fn, seek_fn, buffer_size) = match params {
            IoContextParams::Read {
                read_fn,
                seek_fn,
                buffer_size,
            } => (Some(read_fn), None, seek_fn, buffer_size),
            IoContextParams::Write {
                write_fn,
                seek_fn,
                buffer_size,
            } => (None, Some(write_fn), seek_fn, buffer_size),
            IoContextParams::ReadWrite {
                read_fn,
                write_fn,
                seek_fn,
                buffer_size,
            } => (Some(read_fn), Some(write_fn), seek_fn, buffer_size),
        };

        // Create a temp context with dummy inner
        // We'll properly initialize inner in the initialize method
        let ctx = Self {
            data,
            read_fn,
            write_fn,
            seek_fn,
            inner: NonNull::dangling(), // Temporary value, will be replaced in initialize
        };

        let mut ctx = Box::new(ctx);

        // trampoline for the read function
        extern "C" fn read_callback(
            opaque: *mut std::ffi::c_void,
            buf: *mut u8,
            buf_size: i32,
        ) -> i32 {
            let ctx = unsafe { &mut *(opaque as *mut IoContext) };
            let buffer = unsafe { std::slice::from_raw_parts_mut(buf, buf_size as usize) };

            // if read_fn is None, this shouldn't be called, but handle it gracefully
            let read_fn = match &mut ctx.read_fn {
                Some(read_fn) => read_fn,
                None => return -1,
            };

            match read_fn(&mut ctx.data, buffer) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        // end of file
                        AVError::Eof as i32
                    } else {
                        bytes_read as i32
                    }
                }
                Err(err) => {
                    #[cfg(feature = "tracing")]
                    tracing::error!("IoContext read error: {}", err);

                    match err.raw_os_error() {
                        Some(code) => -code,
                        None => -1,
                    }
                }
            }
        }

        // trampoline for the write function
        extern "C" fn write_callback(
            opaque: *mut std::ffi::c_void,
            buf: *const u8,
            buf_size: i32,
        ) -> i32 {
            let ctx = unsafe { &mut *(opaque as *mut IoContext) };
            let buffer = unsafe { std::slice::from_raw_parts(buf, buf_size as usize) };

            // if write_fn is None, this shouldn't be called, but handle it gracefully
            let write_fn = match &mut ctx.write_fn {
                Some(write_fn) => write_fn,
                None => return -1,
            };

            match write_fn(&mut ctx.data, buffer) {
                Ok(bytes_written) => bytes_written as i32,
                Err(err) => {
                    #[cfg(feature = "tracing")]
                    tracing::error!("IoContext write error: {}", err);

                    match err.raw_os_error() {
                        Some(code) => -code,
                        None => -1,
                    }
                }
            }
        }

        // trampoline for the seek function
        extern "C" fn seek_callback(
            opaque: *mut std::ffi::c_void,
            offset: i64,
            whence: i32,
        ) -> i64 {
            let ctx = unsafe { &mut *(opaque as *mut IoContext) };

            // if seek_fn is None, this shouldn't be called, but handle it gracefully
            let seek_fn = match &mut ctx.seek_fn {
                Some(seek_fn) => seek_fn,
                None => return -1, // error
            };

            let seek_from = match whence {
                libc::SEEK_SET => SeekFrom::Start(offset as u64),
                libc::SEEK_CUR => SeekFrom::Current(offset),
                libc::SEEK_END => SeekFrom::End(offset),
                _ => return -1, // error
            };

            match seek_fn(&mut ctx.data, seek_from) {
                Ok(position) => position as i64,
                Err(_) => -1, // error
            }
        }

        // determine write flag based on provided callbacks
        let write_flag = if ctx.write_fn.is_some() { 1 } else { 0 };

        unsafe {
            // create the avio context with appropriate callbacks
            let buffer_ptr = sys::av_malloc(buffer_size as usize) as *mut _;
            let opaque = ctx.as_mut_ptr() as *mut _ as *mut std::ffi::c_void;

            let context = sys::avio_alloc_context(
                buffer_ptr,         // buffer
                buffer_size as i32, // buffer size
                write_flag,         // write flag
                opaque,             // opaque pointer to our context
                match &ctx.read_fn {
                    Some(_) => Some(read_callback),
                    None => None,
                }, // read callback
                match &ctx.write_fn {
                    Some(_) => Some(write_callback),
                    None => None,
                }, // write callback
                match &ctx.seek_fn {
                    Some(_) => Some(seek_callback),
                    None => None,
                }, // seek callback
            );

            // check if context is null and return error if it is
            ctx.inner = NonNull::new(context).ok_or(crate::Error::Alloc)?;
        }

        // has to be pinned so that opaque ptr to this object doesn't become
        // invalid
        Ok(Box::into_pin(ctx))
    }

    pub fn as_mut_ptr(&mut self) -> *mut sys::AVIOContext {
        self.inner.as_ptr()
    }

    pub fn as_ptr(&self) -> *const sys::AVIOContext {
        self.inner.as_ptr()
    }
}

impl<D> Drop for IoContext<D> {
    fn drop(&mut self) {
        unsafe {
            // frees both the context and the buffer
            let mut ptr = self.inner.as_ptr();
            sys::avio_context_free(&mut ptr);
        }
    }
}
