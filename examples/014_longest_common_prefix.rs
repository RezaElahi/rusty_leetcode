//! Write a function to find the longest common prefix string amongst an array of strings. Strings might be empty.
//! If there is no common prefix, return an empty string "".
//!
//!
struct Solution;

impl Solution {
    /// Build the prefix from common prefix - my first solution
    pub fn longest_common_prefix_1(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_owned();
        }
        // check the common prefix of each two new items
        let mut prefix = strs.first().unwrap().to_owned();
        for s in &strs {
            let mut _prefix = String::new();
            for (c1, c2) in s.chars().zip(prefix.chars()) {
                if c1 == c2 {
                    _prefix.push(c1);
                } else {
                    break;
                }
            }
            if prefix.is_empty() {
                return prefix;
            }
            prefix = _prefix;
        }
        prefix
    }

    /// Horizental scanning: LCP(S1​…Sn​)=LCP(LCP(LCP(S1​,S2​),S3​),…Sn​)
    /// Trim an assumed prefix
    /// Time complexity : O(S) , where S is the sum of all characters in all strings.
    pub fn longest_common_prefix_2(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_owned();
        }

        let mut prefix = strs.first().unwrap().to_owned();

        for s in &strs {
            while !s.starts_with(&prefix) {
                prefix.pop();
                if prefix.is_empty() {
                    return "".to_owned();
                }
            }
        }
        prefix
    }

    /// Vertical scanning -> it was slower!
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_owned();
        }

        let prefix: Vec<char> = strs.first().unwrap().chars().collect();

        for i in 0..prefix.len() {
            let c = prefix.get(i).unwrap();
            for s in strs.iter().skip(1) {
                // if s is big enough, compare its i charachter
                if s.len() == i || s.chars().nth(i).unwrap() != *c {
                    return prefix[0..i].iter().collect();
                }
            }
        }
        strs.first().unwrap().to_owned()
    }

    // TODO: implement other editorial approach
}

fn main() {
    let strs = vec![String::from("cir"), String::from("car")];
    // let strs = vec![String::from("ab"), String::from("abc")];
    dbg!(Solution::longest_common_prefix(strs));
}
