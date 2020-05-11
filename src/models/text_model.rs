use models::errors::TextError;
use models::sigma::Sigma;
use models::symbol::{Symbol, SymbolExt};
use std::iter::FromIterator;
use std::str;

struct Confix {
    confix: Vec<Symbol>,
}

impl Confix {
    pub fn new(marker_symbol: &Symbol, ngram_length: usize) -> Confix {
        let confix: Vec<Symbol> = (0..ngram_length)
            .into_iter()
            .map(|_| marker_symbol.clone())
            .collect::<Vec<Symbol>>();
        Confix { confix }
    }

    pub fn add_to_symbols(&self, symbols: &Vec<Symbol>) -> Vec<Symbol> {
        let mut formated: Vec<Symbol> = self.confix.clone();
        formated.extend(symbols.clone());
        formated.extend(self.confix.clone());
        formated
    }
}

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
