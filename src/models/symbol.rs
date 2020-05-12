use std::str;

/// Deduce the symbols amount of bytes from first byte
///
/// *Note*: This approach is provided by Jean VanCoppenolle. See README for more information.
#[inline]
pub fn char_width(byte: u8) -> usize {
    const TABLE: [usize; 16] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 3, 4];
    TABLE[(byte >> 4) as usize]
}

/// Group bytes of a symbol
///
/// A utf-8 symbol or grapheme cluster can contain multiple bytes, e.g. additional diacritics.
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Symbol {
    pub symbol: Vec<u8>,
}

impl Symbol {
    pub fn from_str(slice: &str) -> Symbol {
        Symbol {
            symbol: slice.as_bytes().to_vec(),
        }
    }

    pub fn from_u8(byte: u8) -> Symbol {
        let mut symbol: Vec<u8> = Vec::new();
        symbol.push(byte);
        Symbol::from_vec_of_u8(symbol)
    }

    pub fn from_vec_of_u8(symbol: Vec<u8>) -> Symbol {
        Symbol { symbol }
    }

    pub fn as_str(&self) -> &str {
        match str::from_utf8(&self.symbol) {
            Ok(slice) => slice,
            Err(err) => panic!("Can't cast to &str because of: {}", err),
        }
    }

    pub fn as_string(&self) -> String {
        String::from(self.as_str())
    }

    pub fn as_bytes_ref(&self) -> &Vec<u8> {
        &self.symbol
    }
}

/// Iterate the symbols of a byte represented text
///
/// * `idx` - iterator position in text
/// * `text` - text as vector of symbols
/// * `text_length` - break condition of iterator
pub struct SymbolIterator {
    pub idx: usize,
    pub text: Vec<u8>,
    pub text_length: usize,
}

impl Iterator for SymbolIterator {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.text_length {
            return None;
        }
        let byte: u8 = self.text[self.idx];
        let width = char_width(byte);
        let offset = self.idx + (width);
        let symbol_bytes = &self.text[self.idx..offset];
        self.idx = offset;
        let symbol = Symbol::from_vec_of_u8(symbol_bytes.to_vec());
        Some(symbol)
    }
}

/// Definition for symbol interface
pub trait SymbolExt {
    fn get_symbols(&self) -> SymbolIterator;
}

/// Implementation of symbol interface for str type
///
/// *Note*: This approach is inspired by Jean VanCoppenolle. See README for more information.
impl SymbolExt for str {
    fn get_symbols(&self) -> SymbolIterator {
        SymbolIterator {
            idx: 0,
            text: self.as_bytes().to_vec(),
            text_length: self.len(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_symbol_extend() {
        assert_eq!("".get_symbols().next(), None);
        let text = "ζoobαr 💖";
        let mut ngrams: SymbolIterator = text.get_symbols();
        assert_eq!(ngrams.next(), Some(Symbol::from_str("ζ")));
        assert_eq!(ngrams.next(), Some(Symbol::from_str("o")));
        assert_eq!(ngrams.next(), Some(Symbol::from_str("o")));
        assert_eq!(ngrams.next(), Some(Symbol::from_str("b")));
        assert_eq!(ngrams.next(), Some(Symbol::from_str("α")));
        assert_eq!(ngrams.next(), Some(Symbol::from_str("r")));
        assert_eq!(ngrams.next(), Some(Symbol::from_str(" ")));
        assert_eq!(ngrams.next(), Some(Symbol::from_str("💖")));
        assert_eq!(ngrams.next(), None);
    }

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
    /// Todo mention Jean
    fn test_char_width() {
        for (i, expected_width) in CORRECT_CHAR_WIDTH.iter().enumerate() {
            if *expected_width > 0 {
                assert_eq!(char_width(i as u8), *expected_width as usize);
            }
        }
    }
}
