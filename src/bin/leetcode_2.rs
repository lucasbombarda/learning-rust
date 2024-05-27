// LeetCode problem:
// https://leetcode.com/problems/palindrome-number

pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        x.to_string().chars().rev().collect::<Vec<char>>() == x.to_string().chars().collect::<Vec<char>>()
    }
}

fn main() {

    let s1 = 121;
    let s2 = -121;
    println!("{} - {}",s1, Solution::is_palindrome(s1));
    println!("{} - {}",s2, Solution::is_palindrome(s2));
}
