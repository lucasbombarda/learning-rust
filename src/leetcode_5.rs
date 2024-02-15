// LeetCode:
// https://leetcode.com/problems/median-of-two-sorted-arrays
//
// If we merge and sort the two arrays it leads to a time complexity of
// O((m+n)log(m+n)). To achieve O(log(m+n)), binary search is needed.
//
// *I didn't implement this alone - special thanks for all the AIs =)

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (a, b) = if nums1.len() <= nums2.len() { (nums1, nums2) } else { (nums2, nums1) };
        let (m, n) = (a.len(), b.len());
        let (mut min_idx, mut max_idx, half_len) = (0, m, (m + n + 1) / 2);

        while min_idx <= max_idx {
            let i = (min_idx + max_idx) / 2;
            let j = half_len - i;
            if i < max_idx && b[j-1] > a[i] {
                min_idx = i + 1;
            } else if i > min_idx && a[i-1] > b[j] {
                max_idx = i - 1;
            } else {
                let max_of_left = if i == 0 {
                    b[j-1] as f64
                } else if j == 0 {
                    a[i-1] as f64
                } else {
                    a[i-1].max(b[j-1]) as f64
                };
                if (m + n) % 2 == 1 { return max_of_left; }

                let min_of_right = if i == m {
                    b[j] as f64
                } else if j == n {
                    a[i] as f64
                } else {
                    a[i].min(b[j]) as f64
                };

                return (max_of_left + min_of_right) / 2.0;
            }
        }

        0.0
    }
}

fn main() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    println!("{}", Solution::find_median_sorted_arrays(nums1.clone(), nums2.clone()));

    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    println!("{}", Solution::find_median_sorted_arrays(nums1, nums2));
}
