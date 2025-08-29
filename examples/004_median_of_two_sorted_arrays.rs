//! Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
//! The overall run time complexity should be O(log (m+n)).
//! 1 <= m + n <= 2000

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }
        // we name the smaller array x
        let len_x = nums1.len();
        let len_y = nums2.len();

        // total_left might be one more than the total_right
        let total_left = len_x + (len_y - len_x).div_ceil(2);

        //
        let mut l_ptr = 0;
        let mut r_ptr = len_x;

        while l_ptr <= r_ptr {
            // these are number of elements which goes into the left partition
            let x_partition = (l_ptr + r_ptr) / 2;
            let y_partition = total_left - x_partition;

            let max_x_left = if x_partition == 0 {
                i32::MIN
            } else {
                nums1[x_partition - 1]
            };
            let min_x_right = if x_partition == len_x {
                i32::MAX
            } else {
                nums1[x_partition]
            };
            let max_y_left = if y_partition == 0 {
                i32::MIN
            } else {
                nums2[y_partition - 1]
            };
            let min_y_right = if y_partition == len_y {
                i32::MAX
            } else {
                nums2[y_partition]
            };

            if max_x_left <= min_y_right && max_y_left <= min_x_right {
                if (len_x + len_y).is_multiple_of(2) {
                    // average of middle values
                    return (std::cmp::max(max_x_left, max_y_left) as f64
                        + std::cmp::min(min_x_right, min_y_right) as f64)
                        / 2.0;
                } else {
                    return std::cmp::max(max_x_left, max_y_left) as f64;
                }
            }
            if max_x_left > min_y_right {
                // make x_partition smaller
                r_ptr = x_partition - 1;
            }
            if max_y_left > min_x_right {
                l_ptr = x_partition + 1;
            }
        }
        panic!("Input arrays are not sorted or invalid");
    }
}

fn main() {
    let nums1 = vec![1, 2, 3, 4, 5, 6, 7];
    let nums2 = vec![12, 13, 14, 15];

    dbg!(Solution::find_median_sorted_arrays(nums1, nums2));
}
