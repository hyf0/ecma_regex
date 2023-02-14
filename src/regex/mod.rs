use std::str::FromStr;
mod r#match;
pub use r#match::*;

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

    /// Returns true if and only if there is a match for the regex in the
    /// string given.
    ///
    /// # Example
    ///
    /// Test if some text contains at least one word with exactly 13
    /// Unicode word characters:
    ///
    /// ```rust
    /// # use ecma_regex::Regex;
    /// # fn main() {
    /// let text = "I categorically deny having triskaidekaphobia.";
    /// assert!(Regex::new(r"\b\w{13}\b").unwrap().is_match(text));
    /// # }
    /// ```
    pub fn is_match(&self, text: &str) -> bool {
        self.is_match_at(text, 0)
    }

    /// Returns the same as is_match, but starts the search at the given offset.
    ///
    /// The significance of the starting point is that it takes the surrounding context into consideration. For example, the `\A` anchor can only match when `start == 0`.
    pub fn is_match_at(&self, text: &str, index: usize) -> bool {
        self.exec(text, index).is_some()
    }

    /// Returns the start and end byte range of the leftmost-first match in
    /// `text`. If no match exists, then `None` is returned.
    ///
    /// Note that this should only be used if you want to discover the position
    /// of the match. Testing the existence of a match is faster if you use
    /// `is_match`.
    ///
    /// # Example
    ///
    /// Find the start and end location of the first word with exactly 13
    /// Unicode word characters:
    ///
    /// ```rust
    /// # use ecma_regex::Regex;
    /// # fn main() {
    /// let text = "I categorically deny having triskaidekaphobia.";
    /// let mat = Regex::new(r"\b\w{13}\b").unwrap().find(text).unwrap();
    /// assert_eq!(mat.start(), 2);
    /// assert_eq!(mat.end(), 15);
    /// # }
    /// ```
    pub fn find<'t>(&self, text: &'t str) -> Option<Match<'t>> {
        self.find_at(text, 0)
    }

    /// Returns the same as find, but starts the search at the given
    /// offset.
    ///
    /// The significance of the starting point is that it takes the surrounding
    /// context into consideration. For example, the `\A` anchor can only
    /// match when `start == 0`.
    pub fn find_at<'t>(&self, text: &'t str, start: usize) -> Option<Match<'t>> {
        let captures = self.exec(text, start)?;
        let start = captures[0];
        let end = captures[1];
        Some(Match::new(text, start, end))
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
