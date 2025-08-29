//! Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place
//!  such that each unique element appears only once. The relative order of the elements should be kept the same.
//! Then return the number of unique elements in nums.
//!

struct Solution;

impl Solution {
    /// Two pointer strategy
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // nums.dedup();

        // i is the plcase where the replacement should occure
        // firt number is always unique.
        let mut i = 1;

        for j in 1..nums.len() {
            // j is the current number
            if nums[j] != nums[i - 1] {
                nums[i] = nums[j];
                i += 1;
            }
        }
        i as i32
    }
}

fn main() {}
