fn linear_search(nums: &Vec<isize>, target: isize) -> isize {
    let mut result = -1;
    for (idx, val) in nums.iter().enumerate() {
        if target == *val {
            result = idx as isize;
            break;
        }
    }
    return result;
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let target = 3;

    let idx = linear_search(&nums, target);

    println!("{} is on index {}", target, idx);
}
