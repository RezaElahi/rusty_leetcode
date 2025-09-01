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

/// Time: O(n) (iterate once, collect once)
/// Space: O(n) (store all characters in rows before flattening)
///
impl Solution {
    //
    pub fn convert_1(s: String, num_rows: i32) -> String {
        if num_rows == 1 || s.len() <= num_rows as usize {
            return s;
        }
        let mut result: Vec<Vec<char>> =
            vec![Vec::with_capacity(s.len() / num_rows as usize); num_rows as usize];

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

        result.into_iter().flatten().collect()
    }

    /// buidling final string row by row
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 || s.chars().count() <= num_rows as usize {
            return s;
        }

        let mut result = String::with_capacity(s.len());
        let chars: Vec<char> = s.chars().collect();

        // cycle length which we select another character for first and last row
        let cycel: usize = (2 * num_rows as usize) - 2;

        for row in 0..num_rows as usize {
            let mut i = row;
            while i < chars.len() {
                //  vertical carachter
                result.push(chars[i]);

                if row != 0 && row != (num_rows as usize - 1) {
                    // these rows contribute two characters in each cycle
                    let diag = (i + cycel) - (2 * row);
                    if diag < chars.len() {
                        result.push(chars[diag]);
                    }
                }
                i += cycel;
            }
        }

        result
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
