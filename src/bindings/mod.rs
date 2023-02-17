pub mod flags;

use libregexp_sys::{lre_compile, lre_exec, lre_get_capture_count};
use std::ffi::{c_char, c_int, CString};

mod error;
pub use error::*;

use self::flags::Flags;

/// The value is copied from the code of QuickJS.
const MAX_COMPILE_ERROR_MSG_LEN: usize = 64;

pub fn compile(pattern: &str, flags: Flags) -> Result<Vec<u8>, CompileError> {
    let mut compiled_byte_code_len: c_int = 0;
    let mut error_msg = [0 as c_char; MAX_COMPILE_ERROR_MSG_LEN];
    let buf_len: usize = pattern.len();
    let buf = CString::new(pattern).map_err(|e| CompileError::Invalid(e))?;

    unsafe {
        let compiled_byte_code = lre_compile(
            (&mut compiled_byte_code_len) as *mut _,
            error_msg.as_mut_ptr(),
            256,
            buf.as_ptr(),
            buf_len,
            flags.bits(),
            std::ptr::null_mut(),
        );
        if compiled_byte_code.is_null() {
            let msg = std::ffi::CStr::from_ptr(error_msg.as_ptr())
                .to_str()
                .unwrap_or("unknown error");
            return Err(CompileError::CompileFailed(msg.to_string()));
        }

        Ok(Vec::from_raw_parts(
            compiled_byte_code,
            compiled_byte_code_len as usize,
            compiled_byte_code_len as usize,
        ))
    }
}

#[derive(Debug)]
pub enum ExecError {
    // The text is too long to be processed by QuickJS.
    TextTooLong,
    // lr_exec returns -1
    Unknown,
}

pub fn exec(
    compiled_byte_code: &[u8],
    text: &str,
    index: usize,
) -> Result<Option<Vec<usize>>, ExecError> {
    assert!(index <= text.len());
    let text_len: c_int = text.len().try_into().map_err(|_| ExecError::TextTooLong)?;

    let capture_count = unsafe { lre_get_capture_count(compiled_byte_code.as_ptr()) } as usize;
    let mut capture = vec![0 as *mut u8; capture_count * 2];
    let ret = unsafe {
        lre_exec(
            capture.as_mut_ptr(),
            compiled_byte_code.as_ptr(),
            text.as_ptr(),
            // safety: index <= text.len() and text.len() <= c_int::MAX
            index.try_into().unwrap_unchecked(),
            text_len,
            0,
            std::ptr::null_mut(),
        )
    };

    match ret {
        1 => Ok(Some(
            capture
                .into_iter()
                .map(|ptr| unsafe { ptr.offset_from(text.as_ptr()) as usize })
                .collect(),
        )),
        0 => Ok(None),
        -1 => Err(ExecError::Unknown),
        _ => unreachable!("unexpected return value: {ret} from `lre_exec` in QuickJS"),
    }
}
