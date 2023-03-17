pub struct Solution{}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut num: String = x.to_string();
        while num.len()>=2{
            let first = num.remove(0);
            if let Some(last) = num.pop(){
                if first!=last{
                    return false;
                }
            }
        }
        return true;
    }

    pub fn is_palindrome_fu(x: i32) -> bool {
        // All negative numbers are not palindrome due to the minus sign
        if x<0{
            return false;
        }
        // All numbers from 0 to 9 are palindrome because they have a single number
        if x<10{
            return true;
        }
        let mut len = 0;
        let mut dividend = 1;
        while x/dividend>=1{
            len = len+1;
            if len==10{
                break;
            }
            dividend = dividend*10;

        }
        let mut forward = 1;
        for backward in (len/2..len+1).rev(){
            // odd number of digits and we are on the middle digit, hence the number is palindrome
            if backward<=forward{
                return true;
            }
            let back_num = get_num_at_pos(&x, backward);
            let front_num = get_num_at_pos(&x, forward);
            if back_num != front_num{
                return false;
            }
            forward = forward+1;
        }
        return true;
    }
}
fn get_num_at_pos(x: &i32, pos: i32) -> i32{
    let mut num= *x;
    // if pos==10, it means that the current digit is the last one, since there are not 11 pos in i32
    if pos<10 {
        let pow: i32 = (10_i32.pow(pos as u32)) as i32;
        // remove all the numbers in position greater than pos
        let mut greater_digits = x / pow;
        greater_digits = greater_digits * pow;
        num = x - greater_digits;
    }

    // remove all the numbers in position lower than pos
    let under_pow: i32 = (10_i32.pow((pos-1) as u32)) as i32;
    num = num/under_pow;
    return num;
}

fn main() {
    let mut pal_num = 3;
    assert!(Solution::is_palindrome(pal_num));
    assert!(Solution::is_palindrome_fu(pal_num));
    pal_num = 323;
    assert!(Solution::is_palindrome(pal_num));
    assert!(Solution::is_palindrome_fu(pal_num));
    pal_num = 3223;
    assert!(Solution::is_palindrome(pal_num));
    assert!(Solution::is_palindrome_fu(pal_num));

    let mut non_pal_num = 13;
    assert!(!Solution::is_palindrome(non_pal_num));
    assert!(!Solution::is_palindrome_fu(non_pal_num));
    let mut non_pal_num = 133;
    assert!(!Solution::is_palindrome(non_pal_num));
    assert!(!Solution::is_palindrome_fu(non_pal_num));
    let mut non_pal_num = 2333;
    assert!(!Solution::is_palindrome(non_pal_num));
    assert!(!Solution::is_palindrome_fu(non_pal_num));

    assert!(Solution::is_palindrome(1000000001));
    assert!(Solution::is_palindrome_fu(1000000001));
}