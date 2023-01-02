# Roman - a Rust crate to deal with roman numbers

This crate allows to convert the integer number to roman numeric strings and vise-versa. It supports only classic roman numbers, which means that it allows maximum 3 instances of the same number one after the other. This means that the biggest number which can be expressed as a roman numeral is 3999. Considering the fact that the number 0 doesn't exist in roman numerics, this crate only supports numbers from 1 to 3999.

```rust
use roman::*;

let roman = Roman::from_string("XII").unwrap();
assert_eq!(roman.as_int(), 12);

let roman = Roman::from_integer(48).unwrap();
assert_eq!(roman.as_string(), String::from("XLVIII"));
```
