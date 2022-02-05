use std::iter::Sum;

fn main() {
    let vec: Vec<i32> = vec![1, 2, 3, 4];
    let sum: i64 = vec.iter().sum();

    println!("{}", sum);
}
