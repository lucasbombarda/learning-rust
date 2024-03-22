struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x.to_string().chars().rev().collect::<Vec<char>>();

        if *x.last().unwrap() == '-' {
            x.pop();
            x.insert(0, '-');
        }

        x.into_iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<i32>()
            .unwrap_or(0)
    }
}

fn main() {
    let x = 1534236469;
    let result = Solution::reverse(x);
    println!("{}", result);

    let x = -123;
    let result = Solution::reverse(x);
    println!("{}", result);

    let x = 120;
    let result = Solution::reverse(x);
    println!("{}", result);
}
