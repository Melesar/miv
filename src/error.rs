use crate::Roman;

/// Errors that can appear when parsing strings or converting from integers
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    /// Empty string is not considered valid roman number
    ///
    /// ```
    /// use miv::*;
    ///
    /// let empty = Roman::from_string("");
    /// assert_eq!(empty, Err(Error::EmptyString));
    /// ```
    #[error("Empty strings are not valid roman numbers")]
    EmptyString,

    /// Some roman numbers cannot appear after some others
    /// For example, the following are invalid: IXI, IVIII, CMD
    ///
    /// ```
    /// use miv::*;
    ///
    /// let invalid = Roman::from_string("IXI");
    /// assert_eq!(invalid, Err(Error::InvalidSequence(IX, I)))
    /// ```
    #[error("{0} cannot be followed by {1}")]
    InvalidSequence(Roman, Roman),

    /// This error appears when the given character is not one of I, V, X, L, C, D, M
    ///
    /// ```
    /// use miv::*;
    ///
    /// let invalid = Roman::from_string("hello");
    /// assert_eq!(invalid, Err(Error::InvalidCharacter(b'h')));
    /// ```
    #[error("{0} is not a valid roman number")]
    InvalidCharacter(u8),

    /// The same number cannot appear more than 3 times in a row 
    ///
    /// ```
    /// use miv::*;
    ///
    /// let invalid = Roman::from_string("MMMMMM");
    /// assert_eq!(invalid, Err(Error::TooManyRepetitions));
    /// ```
    #[error("The same number cannot be more than 3 in a row")]
    TooManyRepetitions,

    /// Zero doesn't exist in the roman numerical system
    ///
    /// ```
    /// use miv::*;
    ///
    /// let zero = Roman::from_integer(0);
    /// assert_eq!(zero, Err(Error::NumberZero));
    /// ```
    #[error("Zero doesn't exist as a roman numeral")]
    NumberZero,

    /// Maximum number is 3999
    ///
    /// ```
    /// use miv::*;
    ///
    /// let zero = Roman::from_integer(4433);
    /// assert_eq!(zero, Err(Error::NumberTooLarge));
    /// ```
    #[error("The given number is too large")]
    NumberTooLarge,
}
