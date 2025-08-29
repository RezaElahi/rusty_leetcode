//! find and item in a sorted array
//!
//!
fn binary_search_iterative(arr: &[i32], target: i32) -> Option<usize> {
    if arr.is_empty() {
        // avoid underflow when arr.len() == 0
        return None;
    }

    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            if mid == 0 {
                // avoid usize underflow when mid = 0
                break;
            }
            // search left half
            high = mid - 1;
        }
    }
    None
}

// divide and conquer
fn binary_search_recursive(arr: &[i32], low: usize, high: usize, target: i32) -> Option<usize> {
    if low > high {
        return None;
    }

    let mid = low + (high - low) / 2;

    if arr[mid] == target {
        Some(mid)
    } else if arr[mid] < target {
        binary_search_recursive(arr, mid + 1, high, target)
    } else {
        if mid == 0 {
            return None; // prevent underflow
        }
        binary_search_recursive(arr, low, mid - 1, target)
    }
}

fn main() {
    let v = vec![1, 2, 4, 7, 9, 12, 15, 20];
    dbg!(binary_search_recursive(v.as_slice(), 0, 7, 20));
}
