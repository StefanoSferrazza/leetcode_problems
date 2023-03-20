pub struct Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut integer = 0;
        let mut char_vec = s.chars();
        let mut current_value = Solution::dict(char_vec.next().expect(format!("expecting at least one char :{}", s).as_str()));
        while let Some(next) = char_vec.next(){
            let next_value = Solution::dict(next);
            if next_value>current_value { integer = integer - current_value; }
            else { integer = integer + current_value; }
            current_value = next_value;
        }
        integer = integer + current_value;
        return integer;
    }

    fn dict(c: char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!("unrecognized char :{}", c)
        }
    }
}

fn main() {
    let mut string: String = String::from("I");
    assert!(1==Solution::roman_to_int(string));
    string = String::from("III");
    assert!(3==Solution::roman_to_int(string));
    string = String::from("XII");
    assert!(12==Solution::roman_to_int(string));
    string = String::from("LVIII");
    assert!(58==Solution::roman_to_int(string));
    string = String::from("MCMXCIV");
    assert!(1994==Solution::roman_to_int(string));
}
