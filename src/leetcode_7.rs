// LeetCode:
// https://leetcode.com/problems/find-polygon-with-the-largest-perimeter

use std::i64;

struct Solution;

impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        nums.sort();

        let mut prev_sum: i64 = 0;
        let mut result = -1;


        for i in nums.iter() {
            if i64::from(*i) < prev_sum {
                result = *i as i64 + prev_sum as i64;
            }
            prev_sum += *i as i64;
        }
        result as i64
    }
}

fn main() {
    let nums = vec![5, 5, 5];
    let result = Solution::largest_perimeter(nums);
    println!("{}", result);

    let nums = vec![1, 12, 1, 2, 5, 50, 3];
    let result = Solution::largest_perimeter(nums);
    println!("{}", result);

    let nums = vec![5, 50, 5];
    let result = Solution::largest_perimeter(nums);
    println!("{}", result);

    let nums = vec![
        300005055, 352368231, 311935527, 315829776, 327065463, 388851949, 319541150, 397875604,
        311309167, 391897750, 366860048, 359976490, 325522439, 390648914, 359891976, 369105322,
        350430086, 398592583, 354559219, 372400239, 344759294, 379931363, 308829137, 335032174,
        336962933, 380797651, 378305476, 336617902, 393487098, 301391791, 394314232, 387440261,
        316040738, 388074503, 396614889, 331609633, 374723367, 380418460, 349845809, 318514711,
        308782485, 308291996, 375362898, 397542455, 397628325, 392446446, 368662132, 378781533,
        372327607, 378737987,
    ];
    let result = Solution::largest_perimeter(nums);
    println!("{}", result); // expected 17876942274
}
