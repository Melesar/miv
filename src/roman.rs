use std::{fmt::Display, str::FromStr};
use crate::prelude::*;


/// Struct representing a roman number
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Roman(u16);

const MAX_REPETITIONS : u8 = 3;

pub const I: Roman = Roman(1);
pub const IV: Roman = Roman(4);
pub const V: Roman = Roman(5);
pub const IX: Roman = Roman(9);
pub const X: Roman = Roman(10);
pub const XL: Roman = Roman(40);
pub const L: Roman = Roman(50);
pub const XC: Roman = Roman(90);
pub const C: Roman = Roman(100);
pub const CD: Roman = Roman(400);
pub const D: Roman = Roman(500);
pub const CM: Roman = Roman(900);
pub const M: Roman = Roman(1000);

const ROMANS_TO_STRINGS: [(Roman, &'static str); 13] = [
    (I, "I"),
    (IV, "IV"),
    (V, "V"),
    (IX, "IX"),
    (X, "X"),
    (XL, "XL"),
    (L, "L"),
    (XC, "XC"),
    (C, "C"),
    (CD, "CD"),
    (D, "D"),
    (CM, "CM"),
    (M, "M"),
];

impl Roman {
    /// Creates an instance of Roman from u16. Accepts only numbers greater then 0 and less then
    /// 4000
    pub fn from_integer(n: u16) -> Result<Self> {
        if n == 0 {
            Err(Error::NumberZero)
        } else if n > 3999 {
            Err(Error::NumberTooLarge)
        } else {
            Ok(Roman(n))
        }
    }

    /// Creates a roman number from a string according to the classic rules.
    pub fn from_string<T: Into<String>>(s: T) -> Result<Self> {
        s.into().parse()
    }

    /// Returns a respective u16 for the given roman number
    pub fn as_int(&self) -> u16 {
        self.0
    }

    /// Returns a string representation for the number
    pub fn as_string(&self) -> String {
        int_to_roman_string(self.0)
    }

    fn prefix_merge(&mut self, prefix: Roman) {
        self.0 -= prefix.0;
    }
}

impl Display for Roman {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl FromStr for Roman {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self> {
        str_to_roman_number(s)
    }
}


fn int_to_roman_string(mut n: u16) -> String {
    let mut result = String::new();

    while n > 0 {
        //Unwrapping here is safe since we know that n > 0
        let (number, str) = ROMANS_TO_STRINGS.iter().rfind(|el| el.0.0 <= n).unwrap(); 
        result.push_str(str);
        n -= number.0;
    }

    result
}

fn str_to_roman_number(s: &str) -> Result<Roman> {
    if s.is_empty() {
        return Err(Error::EmptyString);
    }

    let romans = collect_romans(s)?;

    if romans.len() == 1 {
        return Ok(romans[0]);
    }

    let mut result = romans[0].0;
    let mut num_repetitions = 0;
    for i in 1..romans.len() {
        let first = romans[i - 1];
        let second = romans[i];

        if first == second && { num_repetitions += 1; num_repetitions } >= MAX_REPETITIONS {
            return Err(Error::TooManyRepetitions);
        } else if first != second {
            num_repetitions = 0;
        }

        if is_valid_sequence(first, second) {
            result = result.checked_add(second.0).ok_or(Error::NumberTooLarge)?;
        } else {
            return Err(Error::InvalidSequence(first, second));
        }
    }

    Ok(Roman(result))
}

fn collect_romans(s: &str) -> Result<Vec<Roman>> {
    let mut romans = Vec::new();
    for c in s.bytes() {
        let mut roman = roman_char_to_number(c)?;
        if let Some(previous_num) = romans.pop() {
            if should_merge(previous_num, roman) {
                roman.prefix_merge(previous_num);
            } else {
                romans.push(previous_num);
            }
        }
        romans.push(roman);
    }

    Ok(romans)
}

fn roman_char_to_number(c: u8) -> Result<Roman> {
    match c {
       b'I' => Ok(I),
       b'V' => Ok(V),
       b'X' => Ok(X),
       b'L' => Ok(L),
       b'C' => Ok(C),
       b'D' => Ok(D),
       b'M' => Ok(M),
       _ => Err(Error::InvalidCharacter(c))
    }
}

fn should_merge(prefix: Roman, postfix: Roman) -> bool {
    match prefix {
        I => postfix == V || postfix == X,
        X => postfix == L || postfix == C,
        C => postfix == D || postfix == M,
        _ => false,
    }
}

fn is_valid_sequence(prefix: Roman, postfix: Roman) -> bool {
    if postfix > prefix {
        return false;
    }

    match prefix {
        M | C | X | I => true,
        CM => postfix != C && postfix != D && prefix != postfix,
        XC => postfix != X && postfix != L && prefix != postfix,
        CD => postfix != C && prefix != postfix,
        XL => postfix != X && prefix != postfix,
        D => postfix != prefix && postfix != CD,
        L => postfix != prefix && postfix != XL,
        V => postfix != prefix && postfix != IV,
        IX | IV => false,
        _ => false
    }
}
