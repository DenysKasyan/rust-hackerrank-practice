use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'diagonal_difference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */
fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let mut primary_diagonal_sum = 0;
    let mut secondary_diagonal_sum = 0;

    for i in 0..arr.len() {
        primary_diagonal_sum += arr[i][i];
        secondary_diagonal_sum += arr[i][arr.len() - 1 - i];
    }

    (primary_diagonal_sum - secondary_diagonal_sum).abs()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // Read the size of the square matrix
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Read the 2D array
    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);
    for i in 0..n as usize {
        arr.push(Vec::with_capacity(n as usize));
        arr[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    // Call the diagonal_difference function and write the result to a file
    let result = diagonal_difference(&arr);
    writeln!(&mut fptr, "{}", result).ok();
}
