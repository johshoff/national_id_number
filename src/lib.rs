#[derive(PartialEq, Eq, Debug)]
pub struct Fødselsnummer {
    value: u64,
}

pub enum Type {
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

    pub fn calculate_checksum(&self) -> Option<u64> {
        fn check_digit(partial_sum: u64) -> Option<u64> {
            match 11 - (partial_sum % 11) {
                11 => Some(0),
                10 => None,
                n if n < 10 => Some(n),
                _ => unreachable!("n can't be less than zero after 11 - x mod 11"),
            }
        }

        let mut divisor = 10000000000;
        let mut digits = Vec::with_capacity(10);
        let mut remaining = self.value;
        for _ in 0..9 {
            // get all values except checksum
            let digit = remaining / divisor;
            remaining -= digit * divisor;
            divisor /= 10;
            digits.push(digit);
        }

        let k1 = check_digit(
            vec![3, 7, 6, 1, 8, 9, 4, 5, 2]
                .iter()
                .zip(&digits)
                .map(|(f, d)| (f * d) as u64)
                .sum::<u64>(),
        )?;
        digits.push(k1);

        let k2: u64 = check_digit(
            vec![5, 4, 3, 2, 7, 6, 5, 4, 3, 2]
                .iter()
                .zip(&digits)
                .map(|(f, d)| f * d)
                .sum::<u64>(),
        )?;

        Some(k1 * 10 + k2)
    }

    pub fn is_valid_checksum(&self) -> bool {
        if let Some(calc_checksum) = self.calculate_checksum() {
            let actual_checksum = self.value % 100;

            calc_checksum == actual_checksum
        } else {
            false
        }
    }

    pub fn get_type(&self) -> Type {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use crate::Fødselsnummer;

    fn fs(s: &str) -> Option<Fødselsnummer> {
        Fødselsnummer::from_string(s)
    }

    #[test]
    fn construction() {
        assert_eq!(fs("s"), None);
        assert_eq!(fs("abcdefghijk"), None);
        assert_eq!(fs("00000000000"), Some(Fødselsnummer { value: 0 }));
        assert_eq!(fs("00000000001"), Some(Fødselsnummer { value: 1 }));
        assert_eq!(fs("00000000001x"), None);
        assert_eq!(
            fs("10000000000"),
            Some(Fødselsnummer { value: 10000000000 })
        );
    }

    #[test]
    fn checksum() {
        // test checksum with "explanation" (cd=check_digit). Looking at k1 only (k2 effect cancelled)
        assert_eq!(fs("30110618200").unwrap().calculate_checksum(), Some(35)); // cd=8   11 - 8 = 3
        assert_eq!(fs("30120618200").unwrap().calculate_checksum(), Some(25)); // cd=9   11 - 9 = 2
        assert_eq!(fs("30130618200").unwrap().calculate_checksum(), Some(15)); // cd=10  11 -10 = 1
        assert_eq!(fs("30140618200").unwrap().calculate_checksum(), Some(05)); // cd=0   11 - 0 =11
        assert_eq!(fs("30150618200").unwrap().calculate_checksum(), None); //     cd=1   11 - 1 =10
        assert_eq!(fs("30160618200").unwrap().calculate_checksum(), Some(95)); // cd=2   11 - 2 = 9
        assert_eq!(fs("30170618200").unwrap().calculate_checksum(), Some(85)); // cd=3   11 - 3 = 8
        assert_eq!(fs("30180618200").unwrap().calculate_checksum(), Some(75)); // cd=4   11 - 4 = 7
        assert_eq!(fs("30190618200").unwrap().calculate_checksum(), Some(65)); // cd=5   11 - 5 = 6
        assert_eq!(fs("30100618200").unwrap().calculate_checksum(), Some(45)); // cd=7   11 - 7 = 4
    }

    #[test]
    fn valid_checksum() {
        assert!(fs("02063626662").unwrap().is_valid_checksum());
        assert!(fs("29085114474").unwrap().is_valid_checksum());
        assert!(fs("22038538709").unwrap().is_valid_checksum());
        assert!(fs("31032335430").unwrap().is_valid_checksum());
        assert!(fs("31031670791").unwrap().is_valid_checksum());
        assert!(fs("05061739582").unwrap().is_valid_checksum());
        assert!(fs("25077648065").unwrap().is_valid_checksum());
        assert!(fs("11051602872").unwrap().is_valid_checksum());
        assert!(fs("30110618235").unwrap().is_valid_checksum());
        assert!(fs("07045838387").unwrap().is_valid_checksum());
        assert!(fs("06041579631").unwrap().is_valid_checksum());
        assert!(fs("21016514958").unwrap().is_valid_checksum());

        assert!(!fs("21016514959").unwrap().is_valid_checksum());
    }
}
