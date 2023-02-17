pub mod error;
pub mod r#match;
pub mod matches;

use crate::bindings::{self, flags::Flags};

use self::{error::Error, matches::Matches, r#match::Match};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Regex {
    compiled_byte_code: Vec<u8>,
}

impl Regex {
    pub fn new(pat: &str) -> Result<Self, Error> {
        Self::with_flags(pat, Flags::empty())
    }

    fn with_flags(pat: &str, flags: Flags) -> Result<Self, Error> {
        let compiled_byte_code = bindings::compile(pat, flags).map_err(|e| match e {
            bindings::CompileError::Invalid(e) => Error::Syntax(e.to_string()),
            bindings::CompileError::CompileFailed(e) => Error::Syntax(e),
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

    /// Returns an iterator for each successive non-overlapping match in
    /// `text`, returning the start and end byte indices with respect to
    /// `text`.
    ///
    /// # Example
    ///
    /// Find the start and end location of every word with exactly 13 Unicode
    /// word characters:
    ///
    /// ```rust
    /// # use ecma_regex::Regex;
    /// # fn main() {
    /// let text = "Retroactively relinquishing remunerations is reprehensible.";
    /// let re = Regex::new(r"\b\w{13}\b").unwrap();
    /// let mut iter = re.find_iter(text);
    /// assert_eq!(iter.next().unwrap().as_str(), "Retroactively");
    /// assert_eq!(iter.next().unwrap().as_str(), "relinquishing");
    /// assert_eq!(iter.next().unwrap().as_str(), "remunerations");
    /// assert_eq!(iter.next().unwrap().as_str(), "reprehensible");
    /// assert!(iter.next().is_none());
    /// # }
    /// ```
    pub fn find_iter<'r, 't>(&'r self, text: &'t str) -> Matches<'r, 't> {
        Matches::new(self, text)
    }

    fn exec(&self, text: &str, index: usize) -> Option<Vec<usize>> {
        bindings::exec(&self.compiled_byte_code, text, index).unwrap()
    }
}
