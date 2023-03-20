extern crate core;

pub struct Solution;
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut roman: String = String::from("");
        let mut working_num = num;
        let mut current_digit = working_num/1000;
        if current_digit >0{
           roman.push_str(&Solution::decimal_to_roman(current_digit, 'M', '/', '/'));
        }
        working_num = working_num - (1000 * current_digit);
        current_digit = working_num/100;
        if current_digit >0{
            roman.push_str(&Solution::decimal_to_roman(current_digit, 'C', 'D', 'M'));
        }
        working_num = working_num - (100 * current_digit);
        current_digit = working_num/10;
        if current_digit >0{
            roman.push_str(&Solution::decimal_to_roman(current_digit, 'X', 'L', 'C'));
        }
        working_num = working_num - (10 * current_digit);
        current_digit = working_num/1;
        if current_digit >0{
            roman.push_str(&Solution::decimal_to_roman(current_digit, 'I', 'V', 'X'));
        }
        return roman;
    }
    pub fn decimal_to_roman(num: i32, roman_letter: char, roman_median_letter: char, roman_bigger_letter: char) -> String{
        let mut roman_number = String::from("");
        match num {
            1 => {
                roman_number.push(roman_letter);
            }
            2 => {
                roman_number.push(roman_letter);
                roman_number.push(roman_letter);
            }
            3 => {
                roman_number.push(roman_letter);
                roman_number.push(roman_letter);
                roman_number.push(roman_letter);
            }
            4 => {
                roman_number.push(roman_letter);
                roman_number.push(roman_median_letter);
            }
            5 => {
                roman_number.push(roman_median_letter);
            }
            6 => {
                roman_number.push(roman_median_letter);
                roman_number.push(roman_letter);
            }
            7 => {
                roman_number.push(roman_median_letter);
                roman_number.push(roman_letter);
                roman_number.push(roman_letter);
            }
            8 => {
                roman_number.push(roman_median_letter);
                roman_number.push(roman_letter);
                roman_number.push(roman_letter);
                roman_number.push(roman_letter);
            }
            9 => {
                roman_number.push(roman_letter);
                roman_number.push(roman_bigger_letter);
            }
            _ => {
                panic!("expected number bewteen 0 and 9, instead found {}", num);
            }
        }
        return roman_number;
    }
}

fn main(){
    // print!("{}", Solution::decimal_to_roman(2,'I','V'));
    let mut integer = 1;
    assert!(String::from("I")==Solution::int_to_roman(integer));
    integer = 3;
    assert!(String::from("III")==Solution::int_to_roman(integer));
    integer = 12;
    assert!(String::from("XII")==Solution::int_to_roman(integer));
    integer = 58;
    assert!(String::from("LVIII")==Solution::int_to_roman(integer));
    integer = 1994;
    assert!(String::from("MCMXCIV")==Solution::int_to_roman(integer));
}