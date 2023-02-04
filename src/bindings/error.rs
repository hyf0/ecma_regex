use std::ffi::NulError;

#[derive(Debug)]
pub enum CompileError {
    /// The pattern contains `/0` byte.
    Invalid(NulError),
    CompileFailed(String),
}