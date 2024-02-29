// LeetCode:
// https://leetcode.com/problems/valid-parentheses/

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut chars = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => chars.push(c),
                ')' => {
                    if chars.pop() != Some('(') {
                        return false;
                    }
                },
                ']' => {
                    if chars.pop() != Some('[') {
                        return false;
                    }
                },
                '}' => {
                    if chars.pop() != Some('{') {
                        return false;
                    }
                },
                _ => return false,
            }
        }
        chars.is_empty()
    }
}


fn main() {
    let s = "([)]".to_string();
    let result = Solution::is_valid(s);
    println!("{}", result);
}
