use ffmpeg_sys_next::*;
use libc::{c_void, SEEK_CUR, SEEK_END, SEEK_SET};

use crate::error::FfmpegError;
use crate::smart_object::SmartPtr;

const AVERROR_IO: i32 = AVERROR(EIO);

/// Safety: The function must be used with the same type as the one used to
/// generically create the function pointer
pub(crate) unsafe extern "C" fn read_packet<T: std::io::Read>(
    opaque: *mut libc::c_void,
    buf: *mut u8,
    buf_size: i32,
) -> i32 {
    let ret = (*(opaque as *mut T))
        .read(std::slice::from_raw_parts_mut(buf, buf_size as usize))
        .map(|n| n as i32)
        .unwrap_or(AVERROR_IO);

    if ret == 0 {
        return AVERROR_EOF;
    }

    ret
}

/// Safety: The function must be used with the same type as the one used to
/// generically create the function pointer
pub(crate) unsafe extern "C" fn write_packet<T: std::io::Write>(
    opaque: *mut libc::c_void,
    buf: *const u8,
    buf_size: i32,
) -> i32 {
    (*(opaque as *mut T))
        .write(std::slice::from_raw_parts(buf, buf_size as usize))
        .map(|n| n as i32)
        .unwrap_or(AVERROR_IO)
}

/// Safety: The function must be used with the same type as the one used to
/// generically create the function pointer
pub(crate) unsafe extern "C" fn seek<T: std::io::Seek>(opaque: *mut libc::c_void, offset: i64, mut whence: i32) -> i64 {
    let this = &mut *(opaque as *mut T);

    let seek_size = whence & AVSEEK_SIZE != 0;
    if seek_size {
        whence &= !AVSEEK_SIZE;
    }

    let seek_force = whence & AVSEEK_FORCE != 0;
    if seek_force {
        whence &= !AVSEEK_FORCE;
    }

    if seek_size {
        let Ok(pos) = this.stream_position() else {
            return AVERROR_IO as i64;
        };

        let Ok(end) = this.seek(std::io::SeekFrom::End(0)) else {
            return AVERROR_IO as i64;
        };

        if end != pos {
            let Ok(_) = this.seek(std::io::SeekFrom::Start(pos)) else {
                return AVERROR_IO as i64;
            };
        }

        return end as i64;
    }

    let whence = match whence {
        SEEK_SET => std::io::SeekFrom::Start(offset as u64),
        SEEK_CUR => std::io::SeekFrom::Current(offset),
        SEEK_END => std::io::SeekFrom::End(offset),
        _ => return -1,
    };

    match this.seek(whence) {
        Ok(pos) => pos as i64,
        Err(_) => AVERROR_IO as i64,
    }
}

pub(crate) struct Inner<T: Send + Sync> {
    pub(crate) data: Option<Box<T>>,
    pub(crate) context: SmartPtr<AVFormatContext>,
    _io: SmartPtr<AVIOContext>,
}

pub(crate) struct InnerOptions {
    pub(crate) buffer_size: usize,
    pub(crate) read_fn: Option<unsafe extern "C" fn(*mut c_void, *mut u8, i32) -> i32>,
    pub(crate) write_fn: Option<unsafe extern "C" fn(*mut c_void, *const u8, i32) -> i32>,
    pub(crate) seek_fn: Option<unsafe extern "C" fn(*mut c_void, i64, i32) -> i64>,
    pub(crate) output_format: *const AVOutputFormat,
}

impl Default for InnerOptions {
    fn default() -> Self {
        Self {
            buffer_size: 4096,
            read_fn: None,
            write_fn: None,
            seek_fn: None,
            output_format: std::ptr::null(),
        }
    }
}

impl<T: Send + Sync> Inner<T> {
    pub fn new(data: T, options: InnerOptions) -> Result<Self, FfmpegError> {
        // Safety: av_malloc is safe to call
        let buffer = unsafe {
            SmartPtr::wrap_non_null(av_malloc(options.buffer_size), |ptr| {
                // We own this resource so we need to free it
                av_free(*ptr);
                // We clear the old pointer so it doesn't get freed again.
                *ptr = std::ptr::null_mut();
            })
        }
        .ok_or(FfmpegError::Alloc)?;

        let mut data = Box::new(data);

        // Safety: avio_alloc_context is safe to call, and all the function pointers are
        // valid
        let mut io = unsafe {
            SmartPtr::wrap_non_null(
                avio_alloc_context(
                    buffer.as_ptr() as *mut u8,
                    options.buffer_size as i32,
                    if options.write_fn.is_some() { 1 } else { 0 },
                    data.as_mut() as *mut _ as *mut c_void,
                    options.read_fn,
                    options.write_fn,
                    options.seek_fn,
                ),
                |ptr| {
                    // Safety: the pointer is always valid.
                    if let Some(ptr) = ptr.as_mut() {
                        // We need to free the buffer
                        av_free(ptr.buffer as *mut libc::c_void);

                        // We clear the old pointer so it doesn't get freed again.
                        ptr.buffer = std::ptr::null_mut();
                    }

                    avio_context_free(ptr);
                },
            )
        }
        .ok_or(FfmpegError::Alloc)?;

        // The buffer is now owned by the IO context
        buffer.into_inner();

        let mut context = if options.write_fn.is_some() {
            let mut context = unsafe {
                SmartPtr::wrap(std::ptr::null_mut(), |ptr| {
                    // We own this resource so we need to free it
                    avformat_free_context(*ptr);
                    *ptr = std::ptr::null_mut();
                })
            };

            // Safety: avformat_alloc_output_context2 is safe to call
            let ec = unsafe {
                avformat_alloc_output_context2(
                    context.as_mut(),
                    options.output_format,
                    std::ptr::null(),
                    std::ptr::null_mut(),
                )
            };
            if ec != 0 {
                return Err(FfmpegError::Code(ec.into()));
            }

            if context.as_ptr().is_null() {
                return Err(FfmpegError::Alloc);
            }

            context
        } else {
            // Safety: avformat_alloc_context is safe to call
            unsafe {
                SmartPtr::wrap_non_null(avformat_alloc_context(), |ptr| {
                    // We own this resource so we need to free it
                    avformat_free_context(*ptr);
                    *ptr = std::ptr::null_mut();
                })
            }
            .ok_or(FfmpegError::Alloc)?
        };

        // The io context will live as long as the format context
        context.as_deref_mut().expect("Context is null").pb = io.as_mut_ptr();

        Ok(Self {
            data: Some(data),
            context,
            _io: io,
        })
    }
}

impl Inner<()> {
    /// Empty context cannot be used until its initialized and setup correctly
    pub unsafe fn empty() -> Self {
        Self {
            data: Some(Box::new(())),
            context: unsafe {
                SmartPtr::wrap(std::ptr::null_mut(), |ptr| {
                    // We own this resource so we need to free it
                    avformat_free_context(*ptr);
                    *ptr = std::ptr::null_mut();
                })
            },
            _io: unsafe { SmartPtr::wrap(std::ptr::null_mut(), |_| {}) },
        }
    }

    pub fn open_output(path: &str) -> Result<Self, FfmpegError> {
        let path = std::ffi::CString::new(path).expect("Failed to convert path to CString");

        // Safety: avformat_alloc_output_context2 is safe to call
        let mut this = unsafe { Self::empty() };

        // Safety: avformat_alloc_output_context2 is safe to call
        let ec = unsafe {
            avformat_alloc_output_context2(this.context.as_mut(), std::ptr::null(), std::ptr::null(), path.as_ptr())
        };
        if ec != 0 {
            return Err(FfmpegError::Code(ec.into()));
        }

        // We are not moving the pointer so this is safe
        if this.context.as_ptr().is_null() {
            return Err(FfmpegError::Alloc);
        }

        // Safety: avio_open is safe to call
        let ec = unsafe { avio_open(&mut this.context.as_deref_mut_except().pb, path.as_ptr(), AVIO_FLAG_WRITE) };

        if ec != 0 {
            return Err(FfmpegError::Code(ec.into()));
        }

        this.context.set_destructor(|ptr| unsafe {
            // We own this resource so we need to free it
            avio_closep(&mut (**ptr).pb);
            avformat_free_context(*ptr);
            *ptr = std::ptr::null_mut();
        });

        Ok(this)
    }
}

#[cfg(test)]
#[cfg_attr(all(test, coverage_nightly), coverage(off))]
mod tests {
    use std::ffi::CString;
    use std::io::Cursor;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Once;

    use ffmpeg_sys_next::{av_guess_format, AVSEEK_FORCE};
    use libc::{c_void, SEEK_CUR, SEEK_END};
    use tempfile::Builder;

    use crate::error::FfmpegError;
    use crate::io::internal::{read_packet, seek, write_packet, Inner, InnerOptions, AVERROR_EOF};

    #[test]
    fn test_read_packet_eof() {
        let mut data: Cursor<Vec<u8>> = Cursor::new(vec![]);
        let mut buf = [0u8; 10];

        unsafe {
            let result =
                read_packet::<Cursor<Vec<u8>>>((&raw mut data) as *mut libc::c_void, buf.as_mut_ptr(), buf.len() as i32);

            assert_eq!(result, AVERROR_EOF);
        }
    }

    #[test]
    fn test_write_packet_success() {
        let mut data = Cursor::new(vec![0u8; 10]);
        let buf = [1u8, 2, 3, 4, 5];

        unsafe {
            let result =
                write_packet::<Cursor<Vec<u8>>>((&raw mut data) as *mut _ as *mut c_void, buf.as_ptr(), buf.len() as i32);
            assert_eq!(result, buf.len() as i32);

            let written_data = data.get_ref();
            assert_eq!(&written_data[..buf.len()], &buf);
        }
    }

    #[test]
    fn test_seek_force() {
        let mut cursor = Cursor::new(vec![0u8; 100]);
        let opaque = &mut cursor as *mut _ as *mut c_void;
        assert_eq!(cursor.position(), 0);
        let offset = 10;
        let mut whence = SEEK_CUR | AVSEEK_FORCE;
        let result = unsafe { seek::<Cursor<Vec<u8>>>(opaque, offset, whence) };

        assert_eq!(result, { offset });
        whence &= !AVSEEK_FORCE;
        assert_eq!(whence, SEEK_CUR);
        assert_eq!(cursor.position(), offset as u64);
    }

    #[test]
    fn test_seek_seek_end() {
        let mut cursor = Cursor::new(vec![0u8; 100]);
        let opaque = &mut cursor as *mut _ as *mut libc::c_void;
        let offset = -10;
        let whence = SEEK_END;
        let result = unsafe { seek::<Cursor<Vec<u8>>>(opaque, offset, whence) };

        assert_eq!(result, 90);
        assert_eq!(cursor.position(), 90);
    }

    #[test]
    fn test_seek_invalid_whence() {
        let mut cursor = Cursor::new(vec![0u8; 100]);
        let opaque = &mut cursor as *mut _ as *mut libc::c_void;
        let result = unsafe { seek::<Cursor<Vec<u8>>>(opaque, 0, 999) };

        assert_eq!(result, -1);
        assert_eq!(cursor.position(), 0);
    }

    #[test]
    fn test_avformat_alloc_output_context2_error() {
        static BUF_SIZE_TRACKER: AtomicUsize = AtomicUsize::new(0);
        static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);
        static INIT: Once = Once::new();

        INIT.call_once(|| {
            BUF_SIZE_TRACKER.store(0, Ordering::SeqCst);
            CALL_COUNT.store(0, Ordering::SeqCst);
        });

        unsafe extern "C" fn dummy_write_fn(_opaque: *mut libc::c_void, _buf: *const u8, _buf_size: i32) -> i32 {
            CALL_COUNT.fetch_add(1, Ordering::SeqCst);
            BUF_SIZE_TRACKER.store(_buf_size as usize, Ordering::SeqCst);
            0 // simulate success
        }

        let invalid_format = CString::new("invalid_format").expect("Failed to create CString");
        let options = InnerOptions {
            buffer_size: 4096,
            write_fn: Some(dummy_write_fn),
            output_format: unsafe { av_guess_format(invalid_format.as_ptr(), std::ptr::null(), std::ptr::null()) },
            ..Default::default()
        };
        let data = ();
        let result = Inner::new(data, options);

        assert!(result.is_err(), "Expected an error but got Ok");

        let call_count = CALL_COUNT.load(Ordering::SeqCst);
        assert_eq!(call_count, 0, "Expected dummy_write_fn to not be called.");

        if let Err(error) = result {
            match error {
                FfmpegError::Code(_) => {
                    eprintln!("Expected avformat_alloc_output_context2 error occurred.");
                }
                _ => panic!("Unexpected error variant: {:?}", error),
            }
        }
    }

    #[test]
    fn test_open_output_valid_path() {
        let temp_file = Builder::new()
            .suffix(".mp4")
            .tempfile()
            .expect("Failed to create a temporary file");
        let test_path = temp_file.path();
        let result = Inner::open_output(test_path.to_str().unwrap());

        assert!(result.is_ok(), "Expected success but got error");
    }

    #[test]
    fn test_open_output_invalid_path() {
        let test_path = "";
        let result = Inner::open_output(test_path);

        assert!(result.is_err(), "Expected Err, got Ok");
    }

    #[test]
    fn test_open_output_avformat_alloc_error() {
        let test_path = tempfile::tempdir().unwrap().path().join("restricted_output.mp4");
        let test_path_str = test_path.to_str().unwrap();
        let result = Inner::open_output(test_path_str);
        if let Err(error) = &result {
            eprintln!("Function returned an error: {:?}", error);
        }

        assert!(
            matches!(result, Err(FfmpegError::Code(_))),
            "Expected FfmpegError::Code but received a different error."
        );
    }
}
