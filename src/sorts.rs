mod util;

use util::random::generate_random_array;

fn bubble_sort(arr : &mut Vec<isize>) {
    let n = arr.len();

    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}


fn insertion_sort(arr: &mut Vec<isize>) {
    for i in 0..arr.len() {
        let x = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > x {
            arr.swap(j, j - 1);
            j -= 1;
        }
        arr[j] = x;
    }
}

fn main() {
    let arr =  generate_random_array(10, -100, 100);
    println!("Original array: {:?}", arr);

    insertion_sort(&mut arr.clone());
    println!("insertion_sort - {:#?}", arr);

    bubble_sort(&mut arr.clone());
    println!("bubble_sort - {:#?}", arr);
}
