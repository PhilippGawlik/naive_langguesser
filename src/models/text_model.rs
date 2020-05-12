use models::errors::TextError;
use models::sigma::Sigma;
use models::symbol::{Symbol, SymbolExt};
use std::iter::FromIterator;
use std::str;

/// Holding a symbol sequence as addition for the text
///
/// The confix is meant as prefix and suffix of a text. It marks the beginning and end of the text.
/// The length of the config is related to the specified ngram_length.
///
/// # Fields
///
/// * `confix` - confix symbols as vector
struct Confix {
    confix: Vec<Symbol>,
}

impl Confix {
    /// Init confix
    ///
    /// # Arguments
    ///
    /// * `marker_symbol` - symbol the confix is build from
    /// * `ngram_length` - length of the confix
    pub fn new(marker_symbol: &Symbol, ngram_length: usize) -> Confix {
        let confix: Vec<Symbol> = (0..ngram_length)
            .into_iter()
            .map(|_| marker_symbol.clone())
            .collect::<Vec<Symbol>>();
        Confix { confix }
    }

    /// add confix to a sequence of symbols
    pub fn add_to_symbols(&self, symbols: &Vec<Symbol>) -> Vec<Symbol> {
        let mut formated: Vec<Symbol> = self.confix.clone();
        formated.extend(symbols.clone());
        formated.extend(self.confix.clone());
        formated
    }
}

/// Hold the text
///
/// # Fields
///
/// * `set_confix` - specify addition of confix
/// * `sigma` - text's alphabet
/// * `symbols` - symbols of the text
pub struct TextModel {
    set_confix: Option<Confix>,
    sigma: Sigma,
    symbols: Vec<Symbol>,
}

impl TextModel {
    pub fn new(ngram_length: usize, sigma: &Sigma) -> Result<TextModel, TextError> {
        let set_confix: Option<Confix> = match &sigma.set_marker {
            Some(marker_symbol) => Some(Confix::new(marker_symbol, ngram_length)),
            None => None,
        };
        Ok(TextModel {
            set_confix,
            sigma: sigma.clone(),
            symbols: Vec::new(),
        })
    }

    /// extension of text
    ///
    /// relevant steps are:
    ///
    /// 1. iterate relevant utf-8 symbols
    /// 2. filter symbols not contained in sigma
    pub fn extend(&mut self, text: &str) {
        let extension = text
            .get_symbols()
            .filter_map(|symbol| self.sigma.contains(symbol))
            .collect::<Vec<Symbol>>();
        self.symbols.extend(extension);
    }

    pub fn get_symbols(&self) -> Vec<Symbol> {
        match &self.set_confix {
            Some(confix) => confix.add_to_symbols(&self.symbols),
            None => self.symbols.clone(),
        }
    }

    /// iterate consecutive ngrams of text
    pub fn ngram_iter(&self, ngram_length: usize) -> NGramIterator {
        assert!(ngram_length > 0);
        let symbols: Vec<Symbol> = self.get_symbols();
        let length: usize = symbols.len();
        NGramIterator {
            idx: 0,
            ngram_length,
            text: symbols,
            text_length: length,
        }
    }
}

/// Iterate consecutive text's ngrams of certain length
///
/// # Fields
/// * `idx` - iterator position in text
/// * `ngram_length` - ngram length
/// * `text` - text as vector of symbols
/// * `text_length` - break condition of iterator
pub struct NGramIterator {
    idx: usize,
    ngram_length: usize,
    text: Vec<Symbol>,
    text_length: usize,
}

impl Iterator for NGramIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let offset: usize = self.idx + self.ngram_length;
        if offset > self.text_length {
            return None;
        };
        let ngram_symbols = &self.text[self.idx..offset];
        self.idx += 1;
        Some(String::from_iter(
            ngram_symbols
                .iter()
                .map(|symbol| symbol.as_str())
                .collect::<Vec<&str>>(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use models::sigma::SigmaType;

    #[test]
    fn test_confix() {
        let marker_symbol: Symbol = Symbol::from_u8(35);
        let confix = Confix::new(&marker_symbol, 3);
        let mut symbols: Vec<Symbol> = Vec::new();
        symbols.push(Symbol::from_u8(97));
        let mut result: Vec<Symbol> = Vec::new();
        let formated: Vec<Symbol> = confix.add_to_symbols(&symbols);
        result.push(Symbol::from_u8(35));
        result.push(Symbol::from_u8(35));
        result.push(Symbol::from_u8(35));
        result.push(Symbol::from_u8(97));
        result.push(Symbol::from_u8(35));
        result.push(Symbol::from_u8(35));
        result.push(Symbol::from_u8(35));
        assert_eq!(result, formated);
    }
    #[test]
    fn test_text_model_ngram_iterator() {
        let input = String::from("abc💖");
        let set_marker: Option<u8> = None;
        let sigma: Sigma = Sigma::new(set_marker, SigmaType::AlphaNum);
        let ngram_length: usize = 2;
        let mut text_model = TextModel::new(ngram_length, &sigma).unwrap();
        text_model.extend(&input[..]);
        let mut iter = text_model.ngram_iter(ngram_length);
        assert_eq!(Some(String::from("ab")), iter.next());
        assert_eq!(Some(String::from("bc")), iter.next());
        assert_eq!(None, iter.next());
    }
}
