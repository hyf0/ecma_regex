use std::ops::Range;

/// Match represents a single match of a regex in a haystack.
///
/// The lifetime parameter `'t` refers to the lifetime of the matched text.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Match<'t> {
    text: &'t str,
    start: usize,
    end: usize,
}

impl<'t> Match<'t> {
    /// Returns the starting byte offset of the match in the haystack.
    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns the ending byte offset of the match in the haystack.
    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }

    /// Returns the range over the starting and ending byte offsets of the
    /// match in the haystack.
    #[inline]
    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    /// Returns the matched text.
    #[inline]
    pub fn as_str(&self) -> &'t str {
        &self.text[self.range()]
    }

    /// Creates a new match from the given haystack and byte offsets.
    #[inline]
    pub(crate) fn new(haystack: &'t str, start: usize, end: usize) -> Match<'t> {
        Match {
            text: haystack,
            start,
            end,
        }
    }
}

impl<'t> From<Match<'t>> for &'t str {
    fn from(m: Match<'t>) -> &'t str {
        m.as_str()
    }
}

impl<'t> From<Match<'t>> for Range<usize> {
    fn from(m: Match<'t>) -> Range<usize> {
        m.range()
    }
}
