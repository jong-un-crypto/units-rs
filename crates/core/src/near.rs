use std::num::ParseIntError;

use regex::Regex;

fn parse_str(input: &str) -> Option<String> {
    let unc = Regex::new(r"(?i:u(?i:nc)?)\s*$")
        .unwrap()
        .replace_all(input, "")
        .to_string();
    crate::util::parse(&unc, 24)
}

pub fn parse(input: &str) -> Result<u128, ParseIntError> {
    parse_str(input)
        .expect("Cannot parse string")
        .parse::<u128>()
}

pub fn to_human(input: u128) -> String {
    crate::util::to_human(input, "U", 24, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: [[&str; 3]; 26] = [
        ["1", "1000000000000000000000000", "1 U"],
        [".1000000000000", "100000000000000000000000", "100 mU"],
        ["1,000", "1000000000000000000000000000", "1,000 U"],
        ["1.0", "1000000000000000000000000", "1 U"],
        [
            "1,000,000",
            "1000000000000000000000000000000",
            "1,000,000 U",
        ],
        [
            "1,000,000.000_000_01   ",
            "1000000000000010000000000000000",
            "1,000,000.00000001 U",
        ],
        ["1MU", "1000000000000000000000000000000", "1,000,000 U"],
        ["1kU   ", "1000000000000000000000000000", "1,000 U"],
        ["0.001_101", "1101000000000000000000", "1.101 mU"],
        ["0.000,101", "101000000000000000000", "101 μU"],
        ["1mU", "1000000000000000000000", "1 mU"],
        ["1 milliU", "1000000000000000000000", "1 mU"],
        [" 001      m U    ", "1000000000000000000000", "1 mU"],
        ["1 milliUNC", "1000000000000000000000", "1 mU"],
        ["1 milliU", "1000000000000000000000", "1 mU"],
        ["1 milliunc", "1000000000000000000000", "1 mU"],
        ["1 milli   ", "1000000000000000000000", "1 mU"],
        ["1 m", "1000000000000000000000", "1 mU"],
        ["1μ", "1000000000000000000", "1 μU"],
        ["1micro", "1000000000000000000", "1 μU"],
        ["1nU", "1000000000000000", "1 nU"],
        ["1p", "1000000000000", "1 pU"],
        ["1f", "1000000000", "1 fU"],
        ["1a", "1000000", "1 aU"],
        ["1z", "1000", "1 zU"],
        ["1y", "1", "1 yU"],
    ];

    #[test]
    fn it_works() {
        for line in &DATA {
            let parsed = parse(line[0]).unwrap();
            let expected = line[1];
            assert_eq!(parsed.to_string(), expected);
            assert_eq!(to_human(parsed), line[2]);
        }
    }
}
