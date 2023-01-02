//! A crate for converting numbers into roman numerical strings and vice-versa 
//!
//! For now it supports only the classic roman numbers, which means that number like IIII is
//! considered illegal. The only way to express the number 4 is this: IV. This also means that the
//! largest valid number is MMMCMXCIX (3999).
//!
//! ```
//! use roman::*;
//!
//! let roman = Roman::from_string("XII").unwrap();
//! assert_eq!(roman.as_int(), 12);
//!
//! let roman = Roman::from_integer(48).unwrap();
//! assert_eq!(roman.as_string(), String::from("XLVIII"));
//! ```
mod roman;
mod prelude;
mod error;

pub use self::roman::Roman;
pub use prelude::*;
