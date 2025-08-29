// Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
// The overall run time complexity should be O(log (m+n)).

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let length = nums1.len() + nums2.len();
    let target = length / 2;
    for i in 0..target {
        let mut nums1_ptr = 0usize;
        let mut nums2_ptr = 0usize;
        if let Some(&n) = nums1.get(nums1_ptr) {}
    }
    0.0
    // if length % 2 == 0 {
    //     // find mid
    // } else {
    // }
}

fn main() {
    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    dbg!(find_median_sorted_arrays(nums1, nums2));
}
