use std::iter;
use std::slice;
use std::str;

#[inline]
pub fn char_width(byte: u8) -> usize {
    const TABLE: [usize; 16] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 3, 4];
    TABLE[(byte >> 4) as usize]
}

#[inline]
pub fn char_offsets(text: &str) -> CharOffsets {
    CharOffsets {
        iter: text.as_bytes().iter(),
        step: 0,
        offset: 0,
    }
}

#[derive(Clone, Debug)]
pub struct CharOffsets<'a> {
    iter: slice::Iter<'a, u8>,
    step: usize,
    offset: usize,
}

impl<'a> Iterator for CharOffsets<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        self.iter.nth(self.step).map(|&byte| {
            let width = char_width(byte);
            self.step = width - 1;
            let current_offset = self.offset;
            self.offset += width;
            current_offset
        })
    }
}

#[derive(Debug)]
pub struct CharNgrams<'a> {
    text: &'a str,
    starts: CharOffsets<'a>,
    ends: iter::Skip<CharOffsets<'a>>,
    finished: bool,
}

impl<'a> CharNgrams<'a> {
    #[inline]
    fn next_span(&mut self) -> Option<(usize, usize)> {
        if self.finished {
            return None;
        }

        let end = match self.ends.next() {
            Some(end) => end,
            None => {
                self.finished = true;
                self.text.len()
            }
        };
        self.starts.next().map(|start| (start, end))
    }
}

impl<'a> Iterator for CharNgrams<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<&'a str> {
        self.next_span().map(|(start, end)| &self.text[start..end])
    }
}

pub trait NgramExt {
    fn char_ngrams(&self, n: usize) -> CharNgrams;
}

impl NgramExt for str {
    fn char_ngrams(&self, n: usize) -> CharNgrams {
        assert!(n > 0);
        let starts = char_offsets(self);
        let ends = starts.clone().skip(n);
        CharNgrams {
            text: &self,
            starts,
            ends,
            finished: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::char_width;
    use super::NgramExt;

    static CORRECT_CHAR_WIDTH: [u8; 256] = [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    #[test]
    fn test_char_width() {
        for (i, expected_width) in CORRECT_CHAR_WIDTH.iter().enumerate() {
            if *expected_width > 0 {
                assert_eq!(char_width(i as u8), *expected_width as usize);
            }
        }
    }

    #[test]
    fn test_unigrams() {
        assert_eq!("".char_ngrams(1).next(), None);
        let text = "ζoobαr 💖";
        let mut ngrams = text.char_ngrams(1);
        assert_eq!(ngrams.next(), Some("ζ"));
        assert_eq!(ngrams.next(), Some("o"));
        assert_eq!(ngrams.next(), Some("o"));
        assert_eq!(ngrams.next(), Some("b"));
        assert_eq!(ngrams.next(), Some("α"));
        assert_eq!(ngrams.next(), Some("r"));
        assert_eq!(ngrams.next(), Some(" "));
        assert_eq!(ngrams.next(), Some("💖"));
        assert_eq!(ngrams.next(), None);
    }

    #[test]
    fn test_bigrams() {
        assert_eq!("".char_ngrams(2).next(), None);
        let text = "ζoobαr 💖";
        let mut ngrams = text.char_ngrams(2);
        assert_eq!(ngrams.next(), Some("ζo"));
        assert_eq!(ngrams.next(), Some("oo"));
        assert_eq!(ngrams.next(), Some("ob"));
        assert_eq!(ngrams.next(), Some("bα"));
        assert_eq!(ngrams.next(), Some("αr"));
        assert_eq!(ngrams.next(), Some("r "));
        assert_eq!(ngrams.next(), Some(" 💖"));
        assert_eq!(ngrams.next(), None);
    }

    #[test]
    fn test_trigrams() {
        assert_eq!("".char_ngrams(3).next(), None);
        let text = "ζoobαr 💖";
        let mut ngrams = text.char_ngrams(3);
        assert_eq!(ngrams.next(), Some("ζoo"));
        assert_eq!(ngrams.next(), Some("oob"));
        assert_eq!(ngrams.next(), Some("obα"));
        assert_eq!(ngrams.next(), Some("bαr"));
        assert_eq!(ngrams.next(), Some("αr "));
        assert_eq!(ngrams.next(), Some("r 💖"));
        assert_eq!(ngrams.next(), None);
    }
}
