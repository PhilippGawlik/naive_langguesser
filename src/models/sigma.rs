use std::collections::HashSet;
use std::str;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Symbol {
    symbol: Vec<u8>,
}

impl Symbol {
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


#[derive(Clone)]
pub enum SigmaType {
    AlphaNum,
    Ascii,
    Test,
}

impl SigmaType {
    pub fn get_symbols(&self) -> HashSet<Symbol> {
        match self {
            SigmaType::AlphaNum => {
                let numbers: HashSet<Symbol> = (48..=57)
                    .into_iter()
                    .map(|byte| Symbol::from_u8(byte))
                    .collect();
                let capitals: HashSet<Symbol> = (65..=90)
                    .into_iter()
                    .map(|byte| Symbol::from_u8(byte))
                    .collect();
                let lower: HashSet<Symbol> = (97..=122)
                    .into_iter()
                    .map(|byte| Symbol::from_u8(byte))
                    .collect();
                let mut sigma: HashSet<Symbol> = HashSet::new();
                sigma.extend(numbers);
                sigma.extend(capitals);
                sigma.extend(lower);
                sigma
            }
            SigmaType::Ascii => (0..=127)
                .into_iter()
                .map(|byte| Symbol::from_u8(byte))
                .collect(),
            SigmaType::Test => (97..=99)
                .into_iter()
                .map(|byte| Symbol::from_u8(byte))
                .collect(),
        }
    }
}

#[derive(Clone)]
pub struct Sigma {
    pub set_marker: Option<Symbol>,
    pub type_: SigmaType,
    sigma: HashSet<Symbol>,
}

impl Sigma {
    pub fn new(set_marker_byte: Option<u8>, type_: SigmaType) -> Sigma {
        let mut sigma: HashSet<Symbol> = type_.get_symbols();
        let set_marker: Option<Symbol> = match set_marker_byte {
            Some(marker_byte) => {
                sigma.insert(Symbol::from_u8(marker_byte));
                Some(Symbol::from_u8(marker_byte))
            }
            None => None,
        };
        Sigma {
            set_marker,
            sigma,
            type_,
        }
    }

    pub fn contains(&self, symbol: Symbol) -> Option<Symbol> {
        match self.sigma.contains(&symbol) {
            true => Some(symbol.clone()),
            false => None,
        }
    }

    pub fn as_ref(&self) -> &HashSet<Symbol> {
        &self.sigma
    }

    pub fn as_set(&self) -> HashSet<Symbol> {
        self.sigma
            .iter()
            .map(|symbol| symbol.clone())
            .collect::<HashSet<Symbol>>()
    }

    pub fn as_vec(&self) -> Vec<Symbol> {
        self.sigma
            .iter()
            .map(|symbol| symbol.clone())
            .collect::<Vec<Symbol>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_symbol1() {
        let lhs = Symbol::from_u8(97);
        let rhs = Symbol::from_u8(97);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_symbol2() {
        let lhs = Symbol::from_u8(97);
        let rhs = Symbol::from_u8(98);
        assert_ne!(lhs, rhs);
    }
}
