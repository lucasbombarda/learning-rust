// LeetCode:
// https://leetcode.com/problems/longest-substring-without-repeating-characters/

use std::cmp::max;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut result = 0;
        let mut start = 0;
        let mut end: usize;
        let mut map = HashMap::new();

        for (idx, c) in s.chars().enumerate() {
            if map.contains_key(&c) {
                start = max(*map.get(&c).unwrap() + 1, start);
            }
            map.insert(c, idx);
            end = idx + 1;
            result = max(result, end - start);
        }
        result as i32
    }
}

fn main() {
    let s1 = "abcabcbb".into();
    let s2 = "bbbbb".into();
    let s3 = "pwwkew".into();
    let s4 = "dvdf".into();

    println!("s1 - {}", Solution::length_of_longest_substring(s1));
    println!("s2 - {}", Solution::length_of_longest_substring(s2));
    println!("s3 - {}", Solution::length_of_longest_substring(s3));
    println!("s4 - {}", Solution::length_of_longest_substring(s4));
}
