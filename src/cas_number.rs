use pgx::*;
use rand::Rng;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
use std::fmt::{Display, Error, Formatter};

#[derive(
    Eq,
    PartialEq,
    Ord,
    Hash,
    PartialOrd,
    PostgresType,
    Serialize,
    Deserialize,
    PostgresEq,
    PostgresOrd,
    PostgresHash,
)]
#[inoutfuncs]
pub struct CasNumber(String);

impl Display for CasNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}

impl InOutFuncs for CasNumber {
    fn input(input: &CStr) -> Self
    where
        Self: Sized,
    {
        let string = input.to_str().expect("invalid UTF-8");

        if !validate(string) {
            panic!("invalid CAS Number")
        }

        CasNumber(string.to_string())
    }

    fn output(&self, buffer: &mut StringInfo) {
        buffer.push_str(&format!("{}", self))
    }
}

fn validate(id: &str) -> bool {
    let re = Regex::new(r"^([1-9])(\d{1,6})-(\d{2})-(\d)$").unwrap();

    let caps = match re.captures(id) {
        Some(caps) => caps,
        None => return false,
    };

    let digits = format!(
        "{}{}{}",
        caps.get(1).unwrap().as_str(),
        caps.get(2).unwrap().as_str(),
        caps.get(3).unwrap().as_str()
    );

    let mut checksum = 0;

    for (n, digit) in digits.chars().rev().enumerate() {
        checksum += (n + 1) as u32 * digit.to_digit(10).unwrap();
    }

    checksum % 10 == caps.get(4).unwrap().as_str().parse::<u32>().unwrap()
}

#[pg_extern]
fn random_cas_number() -> CasNumber {
    let base = rand::thread_rng().gen_range(4, 9);
    let mut digits = Vec::new();
    let mut checksum: u32 = 0;

    for i in 0..(base + 2) {
        let digit = random_digit(i == 0);
        checksum += (base as u32 + 2 - i as u32) * digit as u32;

        digits.push(digit)
    }

    let mut string = String::new();

    for (i, digit) in digits.into_iter().enumerate() {
        if i == base as usize {
            string.push_str("-");
        }

        string.push_str(&digit.to_string())
    }

    CasNumber(format!("{}-{}", string, checksum % 10))
}

fn random_digit(exclude_zero: bool) -> u8 {
    if exclude_zero {
        rand::thread_rng().gen_range(1, 9)
    } else {
        rand::thread_rng().gen_range(0, 9)
    }
}
