use crate::Roman;

/// Errors that can appear when parsing strings or converting from integers
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    /// Empty string is not considered valid roman number
    #[error("Empty strings are not valid roman numbers")]
    EmptyString,

    /// Some roman numbers cannot appear after some others
    /// For example, the following are invalid: IXI, IVIII, CMD
    #[error("{0} cannot be followed by {1}")]
    InvalidSequence(Roman, Roman),

    /// This error appears when the given character is not one of I, V, X, L, C, D, M
    #[error("{0} is not a valid roman number")]
    InvalidCharacter(u8),

    /// The same number cannot appear more than 3 times in a row 
    #[error("The same number cannot be more than 3 in a row")]
    TooManyRepetitions,

    /// Zero doesn't exist in the roman numerical system
    #[error("Zero doesn't exist as a roman numeral")]
    NumberZero,

    /// Maximum number is 3999
    #[error("The given number is too large")]
    NumberTooLarge,
}
