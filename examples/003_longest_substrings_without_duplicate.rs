//! Given a string s, find the length of the longest without duplicate characters.
//! s consists of English letters, digits, symbols and spaces.
//!

use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut size = 0;

        let mut chars = HashMap::new();

        let mut left = 0;
        for (right, ch) in s.char_indices() {
            if let Some(&idx) = chars.get(&ch)
                && idx >= left
            {
                left = idx + 1;
            }
            chars.insert(ch, right);
            size = size.max(right - left + 1);
        }

        size as i32
    }

    /// brute force
    pub fn _length_of_longest_substring(s: String) -> i32 {
        let string_length = s.chars().count();

        let mut size = 0;
        let mut substring = String::new();

        for i in 0..string_length {
            for char in s.chars().skip(i) {
                if substring.contains(char) {
                    break;
                } else {
                    substring.push(char);
                }
            }
            size = size.max(substring.chars().count());
            substring.clear();
        }

        size as i32
    }
}

fn main() {
    let s = "abcacdeafb".to_owned();
    dbg!(Solution::length_of_longest_substring(s));
}
