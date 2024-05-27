// LeetCode problem:
// https://leetcode.com/problems/rearrange-array-elements-by-sign/

struct Solution;

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        let mut positives: Vec<i32> = Vec::new();
        let mut negatives: Vec<i32> = Vec::new();
        for val in nums.iter() {
            if *val > 0 {
                positives.push(*val);
            } else {
                negatives.push(*val);
            }
        }

        for i in 0..positives.len() {
            result.push(positives[i]);
            result.push(negatives[i]);
        }

    return result;
    }
}

fn main() {

    let nums = vec![3, 1, -2, -5, 2, -4];
    // let nums = vec![-1, 1];
    let result = Solution::rearrange_array(nums);

    println!("{:?}", result);
}
