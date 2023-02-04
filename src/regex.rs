use crate::{
    bindings::{self, Flags},
    CompileError,
};

pub struct Regex {
    compiled_byte_code: Vec<u8>,
}

impl Regex {
    pub fn new(pat: &str) -> Result<Self, CompileError> {
        let compiled_byte_code = bindings::compile(pat, Flags::empty())?;
        Ok(Regex { compiled_byte_code })
    }

    pub fn is_match(&self, text: &str) -> bool {
        self.exec(text, 0).is_some()
    }

    fn exec(&self, text: &str, index: usize) -> Option<Vec<usize>> {
        bindings::exec(&self.compiled_byte_code, text, index)
    }
}