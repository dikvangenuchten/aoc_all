use std::str::FromStr;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Valve {
    pub name: Name,
    pub flow_rate: u16,
    connects_to: Vec<Name>,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Name(u8, u8);

impl Valve {
    pub fn new<'a>(name: Name, flow_rate: u16, connects_to: Vec<Name>) -> Valve {
        Valve {
            name,
            flow_rate,
            connects_to,
        }
    }

    pub fn from_str(s: &str) -> Valve {
        lazy_static! {
            static ref RE_VALVE: Regex = Regex::new(r"(-?[A-Z]{2})").unwrap();
            static ref RE_FR: Regex = Regex::new(r"(-?\d+)").unwrap();
        }
        let captures: Vec<&str> = RE_VALVE.find_iter(s).map(|s| s.as_str()).collect();
        let (name, others) = captures.split_at(1);

        let connects_to = others.iter().map(|&s| s.into()).collect();

        let flow_rate = RE_FR.find(s).unwrap().as_str().parse::<u16>().unwrap();

        Valve::new(name[0].into(), flow_rate, connects_to)
    }

    pub fn connects_to<'a>(&'a self) -> &'a Vec<Name> {
        &self.connects_to
    }
}

impl From<&str> for Name {
    fn from(value: &str) -> Self {
        debug_assert!(value.len() == 2);
        let (a, b) = value.chars().map(|c| c as u8 - 65).collect_tuple().unwrap();
        debug_assert!(a < 26);
        debug_assert!(b < 26);
        Name(a, b)
    }
}

impl From<u16> for Name {
    fn from(int: u16) -> Self {
        let a = int % 26;
        let b = int / 26;
        Self(a as u8, b as u8)
    }
}

impl Into<u16> for Name {
    fn into(self) -> u16 {
        self.0 as u16 + (self.1 as u16 * 26)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_conversion_from_name_to_u16(
        #[values(
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q",
            "R", "S", "T", "U", "V", "W", "X", "Y", "Z"
        )]
        a: String,
        #[values(
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q",
            "R", "S", "T", "U", "V", "W", "X", "Y", "Z"
        )]
        b: &str,
    ) {
        let name: Name = Name::from(&*(a + b));
        let int: u16 = name.into();
        assert!(int < (26 * 26));
        assert_eq!(name, int.into());
    }
}
