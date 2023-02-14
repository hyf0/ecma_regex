use libregexp_sys::{lre_compile, lre_exec, lre_get_capture_count};
use std::ffi::{c_char, c_int, CString};

mod error;
pub use error::*;

use crate::Flags;

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

pub fn exec(compiled_byte_code: &[u8], text: &str, index: usize) -> Option<Vec<usize>> {
    assert!(index <= text.len());

    let capture_count = unsafe { lre_get_capture_count(compiled_byte_code.as_ptr()) } as usize;
    // TODO: why this require usize instead of c_uchar?
    let mut capture = vec![0 as usize; capture_count * 2];

    let ret = unsafe {
        lre_exec(
            capture.as_mut_ptr() as *mut _,
            compiled_byte_code.as_ptr(),
            text.as_ptr(),
            index.try_into().unwrap(),
            text.len().try_into().unwrap(),
            0,
            std::ptr::null_mut(),
        )
    };

    if ret == 1 {
        Some(
            capture
                .into_iter()
                .map(|ptr| {
                    let offset = unsafe { (ptr as *const u8).offset_from(text.as_ptr()) };
                    offset as usize
                })
                .collect(),
        )
    } else if ret == 0 {
        None
    } else {
        // Errored
        panic!("TODO")
    }
}
