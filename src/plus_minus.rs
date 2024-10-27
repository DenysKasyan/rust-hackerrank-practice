use std::io::{self, BufRead};

/*
 * Complete the 'plus_minus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */
fn plus_minus(arr: &[i32]) {
    let mut positive_count = 0;
    let mut negative_count = 0;
    let mut zero_count = 0;
    let total_count = arr.len() as f64;

    for &value in arr.iter() {
        if value > 0 {
            positive_count += 1;
        } else if value < 0 {
            negative_count += 1;
        } else {
            zero_count += 1;
        }
    }

    println!("{:.6}", positive_count as f64 / total_count);
    println!("{:.6}", negative_count as f64 / total_count);
    println!("{:.6}", zero_count as f64 / total_count);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();
    plus_minus(&arr);
}
