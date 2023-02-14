use std::str::FromStr;

use crate::{
    bindings::{self},
    Flags,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Error {
    InvalidFlag(char),
    SyntaxError(String),
}

impl Error {
    pub fn invalid_flag(flag: char) -> Self {
        Error::InvalidFlag(flag)
    }

    pub fn ecma_literal_must_start_with_slash(got: &str) -> Self {
        Error::SyntaxError(format!("ECMA literal must start with /, got {got}"))
    }

    pub fn ecma_literal_must_have_end_slash() -> Self {
        Error::SyntaxError(format!("ECMA literal must have end /"))
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Regex {
    compiled_byte_code: Vec<u8>,
}

impl Regex {
    pub fn new(pat: &str) -> Result<Self, Error> {
        Self::with_flags(pat, Flags::empty())
    }

    pub fn with_flags(pat: &str, flags: Flags) -> Result<Self, Error> {
        let compiled_byte_code = bindings::compile(pat, flags).map_err(|e| match e {
            bindings::CompileError::Invalid(e) => Error::SyntaxError(e.to_string()),
            bindings::CompileError::CompileFailed(e) => Error::SyntaxError(e),
        })?;
        Ok(Regex { compiled_byte_code })
    }

    pub fn is_match(&self, text: &str) -> bool {
        self.exec(text, 0).is_some()
    }

    fn exec(&self, text: &str, index: usize) -> Option<Vec<usize>> {
        bindings::exec(&self.compiled_byte_code, text, index)
    }

    /// Parse a `Regex` from an ECMA literal.
    ///
    /// # Pitfalls
    ///
    /// If you try to make equivalent [Regex] from JavaScript code.
    ///
    /// There are some pitfalls:
    ///
    /// Make sure using [raw string literals](https://doc.rust-lang.org/reference/tokens.html#raw-string-literals) of Rust for translating JavaScript RegExp literal.
    ///
    /// For example, To translating JavaScript RegExp literal `/\w+/`, you need to write `Regex::from_ecma_literal(r#"/\w+/"#)` instead of `Regex::from_ecma_literal("/\w+/")`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ecma_regex::{Regex, Flags};
    /// assert_eq!(
    ///   Regex::unstable_from_ecma_literal(r#"/\w+/"#),
    ///   Regex::new("\\w+")
    /// );
    /// assert_eq!(
    ///   Regex::unstable_from_ecma_literal(r#"/\w+/g"#),
    ///   Regex::with_flags("\\w+", Flags::GLOBAL)
    /// );
    /// ```
    pub fn unstable_from_ecma_literal(literal: &str) -> Result<Self, Error> {
        if !literal.starts_with('/') {
            return Err(Error::ecma_literal_must_start_with_slash(literal));
        }

        // Fast path
        if literal.ends_with('/') {
            let pat = &literal[1..literal.len() - 1];
            return Self::new(pat);
        }

        let end_slash_pos = literal
            .bytes()
            .enumerate()
            .rev()
            .find_map(|(i, b)| (b == b'/').then_some(i))
            .ok_or_else(|| Error::ecma_literal_must_have_end_slash())?;

        if end_slash_pos == 1 {
            return Err(Error::ecma_literal_must_have_end_slash());
        }

        let flags = if end_slash_pos == literal.len() {
            Flags::empty()
        } else {
            let flags_start = end_slash_pos + 1;
            let flags_str = &literal[flags_start..];
            Flags::from_str(flags_str).map_err(|invalid_flag| Error::invalid_flag(invalid_flag))?
        };

        let pat = &literal[1..end_slash_pos];

        Self::with_flags(pat, flags)
    }
}
