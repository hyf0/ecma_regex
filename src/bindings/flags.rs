use std::{ffi::c_int, str::FromStr};

use bitflags::bitflags;

bitflags! {
    pub struct Flags: c_int {
        /// Global search.
        const GLOBAL = 1 << 0;
        /// Disable Case-insensitive search.
        const IGNORE_CASE = 1 << 1;
        /// Allows `^` and `$` to match newline characters.
        const MULTI_LINE = 1 << 2;
        /// Allows . to match newline characters.
        const DOT_ALL = 1 << 3;
        /// 	"Unicode"; treat a pattern as a sequence of Unicode code points.
        const UTF16 = 1 << 4;
        /// Perform a "sticky" search that matches starting at the current position in the target string.
        const STICKY = 1 << 5;
    }
}

impl FromStr for Flags {
    type Err = String;

    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Regular_Expressions#advanced_searching_with_flags
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut flags = Flags::empty();
        for c in s.chars() {
            match c {
                'g' => flags |= Flags::GLOBAL,
                'i' => flags |= Flags::IGNORE_CASE,
                'm' => flags |= Flags::MULTI_LINE,
                's' => flags |= Flags::DOT_ALL,
                'u' => flags |= Flags::UTF16,
                'y' => flags |= Flags::STICKY,
                _ => return Err(format!("invalid flag: \"{c}\"")),
            }
        }
        Ok(flags)
    }
}


#[test]
fn should_equal() {
    assert_eq!(Flags::GLOBAL.bits as u32, libregexp_sys::LRE_FLAG_GLOBAL);
    assert_eq!(
        Flags::IGNORE_CASE.bits as u32,
        libregexp_sys::LRE_FLAG_IGNORECASE
    );
    assert_eq!(
        Flags::MULTI_LINE.bits as u32,
        libregexp_sys::LRE_FLAG_MULTILINE
    );
    assert_eq!(Flags::DOT_ALL.bits as u32, libregexp_sys::LRE_FLAG_DOTALL);
    assert_eq!(Flags::UTF16.bits as u32, libregexp_sys::LRE_FLAG_UTF16);
    assert_eq!(Flags::STICKY.bits as u32, libregexp_sys::LRE_FLAG_STICKY);

    assert_eq!(
        Flags::all().bits as u32,
        0 | libregexp_sys::LRE_FLAG_GLOBAL
            | libregexp_sys::LRE_FLAG_IGNORECASE
            | libregexp_sys::LRE_FLAG_MULTILINE
            | libregexp_sys::LRE_FLAG_DOTALL
            | libregexp_sys::LRE_FLAG_UTF16
            | libregexp_sys::LRE_FLAG_STICKY
    );
}
