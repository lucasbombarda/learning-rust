fn avg(nums: &Vec<f64>) -> f64 {
    let sum: f64 = nums.iter().sum();
    sum / nums.len() as f64
}

fn median(nums: &Vec<i32>) -> Result<f64, &'static str> {
    if nums.len() == 0 {
        return Err("The vector is empty");
    }

    let mut sorted = nums.clone();
    sorted.sort();

    let sorted_len = sorted.len();

    if sorted_len % 2 == 1 {
        Ok(sorted[sorted_len / 2] as f64)
    } else {
        let mid_1 = sorted[sorted_len / 2 - 1] as f64;
        let mid_2 = sorted[sorted_len / 2] as f64;
        Ok(avg(&vec![mid_1, mid_2]))
    }
}

fn main() {
    let n: Vec<i32> = vec![77, 2, 1, 2, 3, 1, 2, 66, 4, 5];
    let res = median(&n);
    println!("{res:?}");
}
