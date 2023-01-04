# MIV - a Rust crate to deal with roman numbers

This crate is for converting to and from the roman numbers with strong emphasis on correctness. It supports only classic roman numbers, which means that it allows maximum 3 instances of the same number one after the other. This means that the biggest number which can be expressed as a roman numeral is 3999. Considering the fact that the number 0 doesn't exist in roman numerics, this crate only supports numbers from 1 to 3999.


```rust
use roman::*;

let roman = Roman::from_string("XII").unwrap();
assert_eq!(roman.as_int(), 12);

let roman = Roman::from_integer(48).unwrap();
assert_eq!(roman.as_string(), String::from("XLVIII"));
```

MIV applies strict rules on parsing the roman strings. It tries to make sure that the string passed is actually a valid roman numeral, not just a collection of numbers written in a row. For example:

```rust
let valid_143 = Roman::from_string("CXLII");
assert_eq!(valid_143, Ok(Roman(143)));

let invalid_ten = Roman::from_string("IXI");
assert_eq!(invalid_ten, Err(Error::InvalidSequence(Roman::IX, Roman::I)));

let invalid_twenty = Roman::from_string("XVV");
assert_eq!(invalid_twenty, Err(Error::InvalidSequence(Roman::V, Roman::V)));
	
let ones_in_the_beginning = Roman::from_string("IIIX");
assert_eq!(ones_in_the_beginning, Err(Error::InvalidSequence(Roman::I, Roman::IX)));
```

It also has other safeguards, like no more than 3 instances of the same number in a row, no empty strings, no non-roman characters, no number 0.

