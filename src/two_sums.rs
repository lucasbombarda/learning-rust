fn two_sums(arr: Vec<isize>, target: isize) -> Vec<isize> {
    let mut result = Vec::new();
    for (i, valx) in arr.iter().enumerate() {
        for (j, valj) in arr.iter().skip(i + 1).enumerate() {
            if valx + valj == target {
                result.push(i as isize);
                result.push((j + 1 + i) as isize);
                break;
            }
        }
    }
    result
}

fn main() {
    let arr = vec![1, 2, 3, 4, 5];
    let t = 4;

    let res = two_sums(arr, t);
    println!("{:?}", res);
}
