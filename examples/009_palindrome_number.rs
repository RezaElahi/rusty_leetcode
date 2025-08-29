// Given an integer x, return true if x is a , and false otherwise.

struct Solution;

impl Solution {
    pub fn is_str_palindrome(s: &str) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let length = chars.len();
        let mut left = 0;
        // we need to use saturating_sub to prevent panic for empty string
        let mut right: usize = length.saturating_sub(1);

        while left < right {
            if chars[left] != chars[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }

    /// x might be negative
    pub fn is_palindrome_1(x: i32) -> bool {
        let s = x.to_string();
        Self::is_str_palindrome(&s)
    }

    /// i32::MAX = 2_147_483_647
    /// u32::MAX = 4_294_967_295
    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() {
            return false;
        }
        if x == 0 {
            return true;
        }
        if x % 10 == 0 {
            return false;
        }

        let mut num = x;
        // Maybe reverse number gets bigger than i32
        let mut rev: i64 = 0;

        while num != 0 {
            rev = rev * 10 + (num % 10) as i64;
            num /= 10;
        }
        // if overflows, the number is not palindrome
        x == rev as i32
    }
}

fn main() {
    dbg!(Solution::is_palindrome(2_134_567_899));
}
