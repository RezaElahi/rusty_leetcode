//! Two Sums Problem
//! Space and Time Complexity - Better to use space also! We cannot buy time.
//! Given an array of integers `nums`` and an integer `target``, return indices of the two numbers such that
//! they add up to `target`. You may assume that each input would have exactly one solution, and
//! you may not use the same element twice.
//!
//! You can return the answer in any order.
//!

use std::collections::HashMap;

/// brute force
pub fn two_sum_o_n2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, &m) in nums.iter().enumerate() {
        for (j, &n) in nums.iter().skip(i + 1).enumerate() {
            if m + n == target {
                return vec![i as i32, (j + i + 1) as i32];
            }
        }
    }
    // based on constrains, it shouldnt reach here
    vec![]
}

/// O(n) for both time and space
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new(); // O(n)
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num; // O(1)
        if let Some(&j) = map.get(&complement) {
            return vec![i as i32, j as i32]; // O(1)
        }
        map.insert(num, i);
    }
    vec![]
}

fn main() {
    let nums = vec![3, 2, 4];
    let target = 6;

    dbg!(two_sum(nums, target));
}
