//! You want to check whether some substring reads the same forward and backward.
//! A palindrome is symmetric around its center.
//! The center can be:
//!     A single character (odd-length palindrome, like "aba").
//!     A gap between two characters (even-length palindrome, like "abba").

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut max_length = 1;
        let mut start = 0;

        for i in 0..chars.len() {
            let (odd_start, odd_length) = Self::expand(&chars, i as isize, i as isize);
            let (even_start, even_length) = Self::expand(&chars, i as isize, (i + 1) as isize);

            if odd_length > max_length {
                start = odd_start;
                max_length = odd_length;
            }

            if even_length > max_length {
                start = even_start;
                max_length = even_length;
            }
        }

        dbg!(&max_length);
        chars[start..(start + max_length)].iter().collect()
    }

    /// expand from left and right, and return the palindrome start index and length
    pub fn expand(chars: &[char], mut left: isize, mut right: isize) -> (usize, usize) {
        let n = chars.len();

        while left >= 0 && right < n as isize && chars[left as usize] == chars[right as usize] {
            left -= 1;
            right += 1;
        }

        ((left + 1) as usize, (right - left - 1) as usize)
    }

    /// An efficient and idiomatic way to cehck palindromic
    pub fn is_palindrome(s: &str) -> bool {
        let chars: Vec<char> = s.chars().collect();

        let n = chars.len();
        let mut left = 0;
        let mut right = n - 1;

        while left < right {
            if chars[left] != chars[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}

fn main() {
    let s = String::from("abbdcdcaeeeea");
    dbg!(Solution::longest_palindrome(s));
    dbg!(Solution::is_palindrome("aca"));
    dbg!(Solution::is_palindrome("abccba"));
    dbg!(Solution::is_palindrome("abccbas"));
}
