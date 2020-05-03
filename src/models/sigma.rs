use std::collections::HashSet;

#[derive(Clone)]
pub enum SigmaType {
    AlphaNum,
    Ascii,
    Test,
}

impl SigmaType {
    pub fn as_bytes(&self) -> HashSet<u8> {
        match self {
            SigmaType::AlphaNum => {
                let numbers: HashSet<u8> = (48..=57).into_iter().collect();
                let capitals: HashSet<u8> = (65..=90).into_iter().collect();
                let lower: HashSet<u8> = (97..=122).into_iter().collect();
                let mut sigma: HashSet<u8> = HashSet::new();
                sigma.extend(numbers);
                sigma.extend(capitals);
                sigma.extend(lower);
                sigma
            }
            SigmaType::Ascii => (0..=127).into_iter().collect(),
            SigmaType::Test => (97..=99).into_iter().collect(),
        }
    }
}

#[derive(Clone)]
pub struct Sigma {
    pub set_marker: Option<u8>,
    pub type_: SigmaType,
    sigma: HashSet<u8>,
}

impl Sigma {
    pub fn new(set_marker: Option<u8>, type_: SigmaType) -> Sigma {
        let mut sigma: HashSet<u8> = type_.as_bytes();
        match set_marker {
            Some(byte) => sigma.insert(byte),
            None => false,
        };
        Sigma {
            set_marker,
            sigma,
            type_,
        }
    }

    pub fn contains(&self, byte: &u8) -> Option<u8> {
        match self.sigma.contains(byte) {
            true => Some(byte.clone()),
            false => None,
        }
    }

    pub fn as_bytes_ref(&self) -> &HashSet<u8> {
        &self.sigma
    }

    pub fn as_set(&self) -> HashSet<String> {
        self.as_bytes_ref()
            .iter()
            .map(|b| (*b as char).to_string())
            .collect::<HashSet<String>>()
    }

    pub fn as_vec(&self) -> Vec<String> {
        self.as_bytes_ref()
            .iter()
            .map(|b| (*b as char).to_string())
            .collect::<Vec<String>>()
    }
}
