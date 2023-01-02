use roman::*;

#[test]
fn valid_numbers_are_parsed_correctly() {
    assert_valid_num("I", 1);
    assert_valid_num("II", 2);
    assert_valid_num("III", 3);
    assert_valid_num("IV", 4);
    assert_valid_num("V", 5);
    assert_valid_num("VI", 6);
    assert_valid_num("VII", 7);
    assert_valid_num("VIII", 8);
    assert_valid_num("IX", 9);
    assert_valid_num("X", 10);
    assert_valid_num("XVII", 17);
    assert_valid_num("XXXIV", 34);
    assert_valid_num("XXXIX", 39);
    assert_valid_num("XL", 40);
    assert_valid_num("XLIX", 49);
    assert_valid_num("L", 50);
    assert_valid_num("LXXIX", 79);
    assert_valid_num("XC", 90);
    assert_valid_num("XCV", 95);
    assert_valid_num("C", 100);
    assert_valid_num("CXV", 115);
    assert_valid_num("CLII", 152);
    assert_valid_num("CXCVII", 197);
    assert_valid_num("CCLIX", 259);
    assert_valid_num("CD", 400);
    assert_valid_num("CDLXXVII", 477);
    assert_valid_num("D", 500);
    assert_valid_num("DCCCLI", 851);
    assert_valid_num("CM", 900);
    assert_valid_num("CMIX", 909);
    assert_valid_num("M", 1000);
    assert_valid_num("MMCDXXI", 2421);
    assert_valid_num("MMMCMXCIX", 3999);

}

#[test]
fn non_roman_strings_are_invalid() {
    assert_empty_string("");
    assert_invalid_character("aaa");
    assert_invalid_character("hello");
    assert_invalid_character("xii");
    assert_invalid_character("іваіллуш");
    assert_invalid_character("QXUI");
    assert_invalid_character("KDE");
}


#[test]
fn smaller_numbers_cannot_precede_larger_ones() {
    assert_invalid_sequence("XXCD");
    assert_invalid_sequence("IIVX");
    assert_invalid_sequence("VX");
    assert_invalid_sequence("VXII");
    assert_invalid_sequence("VXII");
}

#[test]
fn some_numbers_are_not_allowed_to_come_one_after_the_other(){
    assert_invalid_sequence("VV");
    assert_invalid_sequence("IXIX");
    assert_invalid_sequence("IVIV");
    assert_invalid_sequence("XCXC");
    assert_invalid_sequence("XCL");
    assert_invalid_sequence("LXLVII");
    assert_invalid_sequence("DCDV");
    assert_invalid_sequence("CXXIXV");
    assert_invalid_sequence("IVI");
    assert_invalid_sequence("IXIV");
    assert_invalid_sequence("XCXVI");
    assert_invalid_sequence("VIV");
    assert_invalid_sequence("DCD");
    assert_invalid_sequence("CMD");
    assert_invalid_sequence("CDC");
    assert_invalid_sequence("XCL");
}

#[test]
fn over_three_same_numbers_in_a_row_are_invalid() {
    assert_too_many_repetitions("MMMDXXXXXXX");
    assert_too_many_repetitions("VIIII");
    assert_too_many_repetitions("CCCC");
}

#[test]
fn numbers_are_converted_into_valid_roman_strings() {
    assert_string_from_number(2, "II");
    assert_string_from_number(5, "V");
    assert_string_from_number(9, "IX");
    assert_string_from_number(15, "XV");
    assert_string_from_number(19, "XIX");
    assert_string_from_number(37, "XXXVII");
    assert_string_from_number(40, "XL");
    assert_string_from_number(53, "LIII");
    assert_string_from_number(60, "LX");
    assert_string_from_number(99, "XCIX");
    assert_string_from_number(128, "CXXVIII");
}

#[test]
fn roman_struct_cannot_be_constructed_from_zero() {
    assert_error(Roman::from_integer(0), Error::NumberZero);
}

#[test]
fn the_largest_valid_number_is_3999() {
    let roman = Roman::from_integer(3999);
    assert!(roman.is_ok());

    assert_error(Roman::from_integer(4000), Error::NumberTooLarge);
}

fn assert_invalid_sequence(s: &str) {
    let res = Roman::from_string(s);
    assert!(res.is_err());
    assert!({
        if let Error::InvalidSequence(_, _) = res.unwrap_err()  {
            true
        } else {
            false
        }
    });
}

fn assert_too_many_repetitions(s: &str) {
    assert_error(Roman::from_string(s), Error::TooManyRepetitions);
}

fn assert_empty_string(s: &str) {
    assert_error(Roman::from_string(s), Error::EmptyString);
}

fn assert_invalid_character(s: &str) {
    let res = Roman::from_string(s);
    assert!(res.is_err());
    assert!({
        if let Error::InvalidCharacter(_) = res.unwrap_err()  {
            true
        } else {
            false
        }
    });
}

fn assert_valid_num(s: &str, value: u16) {
    assert_eq!(Roman::from_string(s).unwrap().as_int(), value); 
}

fn assert_string_from_number(n: u16, value: &str) {
    let result = Roman::from_integer(n);
    assert!(result.is_ok());

    let roman_string = result.unwrap().as_string();
    assert_eq!(roman_string, String::from(value));
}

fn assert_error(res: Result<Roman>, err: Error) {
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), err);
}
