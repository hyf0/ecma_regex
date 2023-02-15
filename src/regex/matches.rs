use crate::{Regex, Match};


/// An iterator over all non-overlapping matches for a particular string.
/// 
/// The iterator yields a Match value. The iterator stops when no more matches can be found.
///
/// 'r is the lifetime of the compiled regular expression and 't is the lifetime of the matched string.
#[derive(Debug)]
pub struct Matches<'r, 't> {
    regex: &'r Regex,
    text: &'t str,
    last_index: usize,
}

impl<'r, 't> Matches<'r, 't> {
    pub(crate) fn new(regex: &'r Regex, text: &'t str) -> Matches<'r, 't> {
        Matches {
            regex,
            text,
            last_index: 0,
        }
    }
}

impl<'r, 't> Iterator for Matches<'r, 't> {
    type Item = Match<'t>;

    fn next(&mut self) -> Option<Match<'t>> {
        let matched = self.regex.find_at(self.text, self.last_index)?;
        self.last_index = matched.end();
        Some(matched)
    }
}