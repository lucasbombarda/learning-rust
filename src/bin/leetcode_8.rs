// LeetCode:
// https://leetcode.com/problems/container-with-most-water/

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_area = 0;

        while left < right {
            let area = (right - left) as i32 * height[left].min(height[right]);
            max_area = max_area.max(area);

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_area
    }
}


fn main() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let result = Solution::max_area(height);
    println!("{}", result);
}
