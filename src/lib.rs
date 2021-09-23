#[derive(PartialEq, Eq, Debug)]
struct Fødselsnummer {
    value: u64,
}

enum Type {
    Normal,
    DNumber,
    HNumber,
}

impl Fødselsnummer {
    pub fn from_string(s: &str) -> Option<Fødselsnummer> {
        if s.len() == 11 {
            s.parse::<u64>().ok().map(|value| Fødselsnummer { value })
        } else {
            None
        }
    }

    pub fn is_valid_checksum(&self) -> bool {
        unimplemented!();
    }

    pub fn get_type(&self) -> Type {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use crate::Fødselsnummer;

    #[test]
    fn construction() {
        assert_eq!(Fødselsnummer::from_string("s"), None);
        assert_eq!(Fødselsnummer::from_string("abcdefghijk"), None);
        assert_eq!(Fødselsnummer::from_string("00000000000"), Some(Fødselsnummer { value: 0 }));
        assert_eq!(Fødselsnummer::from_string("00000000001"), Some(Fødselsnummer { value: 1 }));
        assert_eq!(Fødselsnummer::from_string("00000000001x"), None);
        assert_eq!(Fødselsnummer::from_string("10000000000"), Some(Fødselsnummer { value: 10000000000 }));
    }
}
