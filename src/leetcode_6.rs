// LeetCode:
// https://leetcode.com/problems/remove-duplicates-from-sorted-array/

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut write_index = 1;
        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                nums[write_index] = nums[i];
                write_index += 1;
            }
        }
        nums.truncate(write_index);
        write_index as i32
    }
}

fn main() {
    let mut nums = vec![0,0,1,1,1,2,2,3,3,4];

    let res = Solution::remove_duplicates(&mut nums);

    println!("{}", res);
}
