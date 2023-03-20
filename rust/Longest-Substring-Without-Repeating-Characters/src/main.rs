extern crate core;

use std::collections::{HashMap, HashSet};

pub struct Slow_Solution;
impl Slow_Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut upper_bound = s.len();
        let mut lower_bound = 0;
        let mut max = 0;
        while upper_bound>lower_bound+1{
            let middle = ((upper_bound-lower_bound)/2)+lower_bound;
            if Slow_Solution::exists_substring_with_n_non_rep_chars(&s, middle){
                max = middle;
                lower_bound = middle;
            }
            else {
                upper_bound = middle;
            }
        }
        if Slow_Solution::exists_substring_with_n_non_rep_chars(&s, max+1){
            max = max + 1;
        }
        return max as i32;
    }

    // This methods takes in input a String s and an usize n.
    // Returns true if there is a substring s'\in s of size n, where each char in s is unique.
    // Returns false otherwise.
    pub fn exists_substring_with_n_non_rep_chars(s: &String,n: usize) -> bool {
        if s.len()<n{
            return false;
        }
        let (mut start, mut end) = (0_usize,n);
        while end<=(s.len()){
            let substring = &s[start..end];
            if Slow_Solution::contains_only_different_chars(substring)==true{
                return true;
            }
            start = start + 1;
            end = end + 1;
        }
        return false;
    }

    // This method takes a slice of chars and returns true iff each char c in the slice is different
    pub fn contains_only_different_chars(str: &str) -> bool {
        let mut char_set: HashSet<char> = HashSet::new();
        for c in str.chars(){
            if char_set.contains(&c){
                return false;
            }
            char_set.insert(c);
        }
        return true;
    }
}

pub struct Solution;
impl Solution{
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max =0;
        let mut start = 0;
        let mut map: HashMap<char,usize> = HashMap::with_capacity(s.len());
        for (i,c ) in s.chars().enumerate(){
            if map.contains_key(&c){
                let end = i;
                let curr = end-start;
                if max<curr{
                    max = curr;
                }
                let start=*map.get(&c).expect("")+1;
                map.remove(&c);
            }
            map.insert(c,i);
        }
        let curr = s.len()-start;
        // if curr>max{
        //     max = curr;
        // }
        return max as i32;
    }
}

fn main() {
    let sol = Slow_Solution::length_of_longest_substring(String::from("qwertyuiopasdfghjklzxcvbnm1234567890"));
    let sol2 = Solution::length_of_longest_substring(String::from("qwertyuiopasdfghjklzxcvbnm1234567890"));
    println!("{}",sol);
    println!("{}",sol2);
}