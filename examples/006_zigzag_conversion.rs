//! write a string in a zigzag pattern in a number of rows
//! s consists of English letters (lower-case and upper-case), ',' and '.
//!
//! Input: s = "PAYPALISHIRING", numRows = 4
//! Output: "PINALSIGYAHRPI"
//! Explanation:
//! P     I    N
//! A   L S  I G
//! Y A   H R
//! P     I
//!

struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut result: Vec<String> = vec![String::new(); num_rows as usize];

        let mut increment: i32 = 1;
        let mut idx: usize = 0;
        for ch in s.chars() {
            result[idx].push(ch);

            if idx == 0 {
                increment = 1;
            } else if idx == (num_rows - 1) as usize {
                increment = -1;
            }

            idx = (idx as i32 + increment) as usize
        }

        result.into_iter().collect()
    }
}

fn main() {
    assert_eq!(
        Solution::convert(String::from("PAYPALISHIRING"), 1),
        "PAYPALISHIRING".to_string()
    );
    assert_eq!(
        Solution::convert(String::from("PAYPALISHIRING"), 3),
        "PAHNAPLSIIGYIR".to_string()
    );
    assert_eq!(
        Solution::convert(String::from("PAYPALISHIRING"), 4),
        "PINALSIGYAHRPI".to_string()
    );
    assert_eq!(Solution::convert(String::from("a"), 2), "a".to_string());
}
