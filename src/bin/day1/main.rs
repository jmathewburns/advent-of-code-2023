use crate::calibration::parse_digits;

pub fn main() {
    let string = include_str!("input.txt");
    let sum = string.lines()
        .map(|str| parse_digits(str)
            .unwrap_or_else(|| panic!("could not parse line: {str}")))
        .sum::<u32>();
    println!("{sum}");
}

pub mod calibration {
    use once_cell::sync::Lazy;
    use regex::{Captures, Regex};

    static WORDS: &str = "(one|two|three|four|five|six|seven|eight|nine)";

    static DIGITS_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(format!("((?<as_word>{WORDS})|(?<as_digit>[0-9]))").as_str()).unwrap());
    static DIGITS_PATTERN_REV: Lazy<Regex> = Lazy::new(|| Regex::new(format!("((?<as_word>{})|(?<as_digit>[0-9]))", reverse_str(WORDS)).as_str()).unwrap());

    pub fn parse_digits(text: &str) -> Option<u32> {
        Some(first_digit(text)? * 10 + last_digit(text)?)
    }

    pub fn first_digit(str: &str) -> Option<u32> {
        DIGITS_PATTERN.captures(str)
            .and_then(convert_digit)
    }

    fn convert_digit(captures: Captures) -> Option<u32> {
        if let Some(word) = captures.name("as_word") {
            word_to_digit(word.as_str())
        } else if let Some(digit) = captures.name("as_digit") {
            digit.as_str().parse().ok()
        } else {
            None
        }
    }

    fn word_to_digit(word: &str) -> Option<u32> {
        match word {
            "one" => Some(1),
            "two" => Some(2),
            "three" => Some(3),
            "four" => Some(4),
            "five" => Some(5),
            "six" => Some(6),
            "seven" => Some(7),
            "eight" => Some(8),
            "nine" => Some(9),
            _ => None
        }
    }

    pub fn last_digit(str: &str) -> Option<u32> {
        let str_rev = reverse_str(str);
        let found = DIGITS_PATTERN_REV.find(str_rev.as_str())?;

        let found_rev = reverse_str(found.as_str());

        if found_rev.chars().next()?.is_ascii_digit() {
            found_rev.parse().ok()
        } else {
            word_to_digit(found_rev.as_str())
        }
    }

    fn reverse_str(str: &str) -> String {
        str.chars().rev().collect()
    }

    #[cfg(test)]
    mod test {
        use phf::phf_map;
        use crate::calibration::parse_digits;

        #[test]
        fn should_parse_correctly() {
            let test_cases: phf::Map<&str, u32> = phf_map! {
                "two1nine" =>  29,
                "eightwothree" => 83,
                "abcone2threexyz" =>  13,
                "xtwone3four" => 24,
                "zoneight234" => 14,
                "7pqrstsixteen" =>  76,
                "lhd8two7dhthqhbzvlknvtrlfthreeninethreetwonez" => 81
            };

            for (key, value) in test_cases.entries() {
                assert_eq!(Some(*value), parse_digits(key));
            }
        }
    }
}

