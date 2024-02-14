pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {

        let mut result = 0;

        let mut prev = 0;
        for c in s.chars() {
            let val = match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0
            };

            if prev < val {
                result += val - 2 * prev;
            } else {
                result += val;
            }

            prev = val;
        }
        result
    }
}

fn main() {
    let s1 = Solution::roman_to_int("MCMXCIV".into());

    println!("MCMXCIV = {}", s1);

    let s2 = Solution::roman_to_int("III".into());

    println!("III = {}", s2);

    let s3 = Solution::roman_to_int("LVIII".into());
    println!("LVIII = {}", s3)

}
