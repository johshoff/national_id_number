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

    pub fn calculate_checksum(&self) -> u64 {
        fn check_number(digit: u64) -> u64 {
            match digit {
                0 => 0,
                n => 11 - n,
            }
        }

        let mut divisor = 10000000000;
        let mut digits = vec![];
        let mut remaining = self.value;
        for _ in 0..9 {
            // get all values except checksum
            let digit = remaining / divisor;
            remaining -= digit * divisor;
            divisor /= 10;
            digits.push(digit);
        }

        let k1 = check_number(
            vec![3, 7, 6, 1, 8, 9, 4, 5, 2]
                .iter()
                .zip(&digits)
                .map(|(f, d)| (f * d) as u64)
                .sum::<u64>()
                % 11,
        );
        digits.push(k1);

        let k2: u64 = check_number(
            vec![5, 4, 3, 2, 7, 6, 5, 4, 3, 2]
                .iter()
                .zip(&digits)
                .map(|(f, d)| f * d)
                .sum::<u64>()
                % 11,
        );

        k1 * 10 + k2
    }

    pub fn is_valid_checksum(&self) -> bool {
        let calc_checksum = self.calculate_checksum();
        let actual_checksum = self.value % 100;

        calc_checksum == actual_checksum
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

    #[test]
    fn checksum() {
        assert!(Fødselsnummer::from_string("02063626662").unwrap().is_valid_checksum());
        assert!(Fødselsnummer::from_string("29085114474").unwrap().is_valid_checksum());
        assert!(Fødselsnummer::from_string("22038538709").unwrap().is_valid_checksum());
        assert!(Fødselsnummer::from_string("31032335430").unwrap().is_valid_checksum());
        assert!(Fødselsnummer::from_string("31031670791").unwrap().is_valid_checksum());
        assert!(Fødselsnummer::from_string("05061739582").unwrap().is_valid_checksum());
        assert!(Fødselsnummer::from_string("25077648065").unwrap().is_valid_checksum());
        assert!(Fødselsnummer::from_string("11051602872").unwrap().is_valid_checksum());
        assert!(Fødselsnummer::from_string("30110618235").unwrap().is_valid_checksum());
        assert!(Fødselsnummer::from_string("07045838387").unwrap().is_valid_checksum());
        assert!(Fødselsnummer::from_string("06041579631").unwrap().is_valid_checksum());
        assert!(Fødselsnummer::from_string("21016514958").unwrap().is_valid_checksum());

        assert!(!Fødselsnummer::from_string("21016514959").unwrap().is_valid_checksum());
    }
}
