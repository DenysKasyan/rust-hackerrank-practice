use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'divisible_sum_pairs' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER k
 *  3. INTEGER_ARRAY ar
 */
fn divisible_sum_pairs(_n: i32, k: i32, ar: &[i32]) -> i32 {
    let mut count = 0;
    for i in 0..ar.len() {
        for j in (i + 1)..ar.len() {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // Read the first multiple input
    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();
    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    // Read the array
    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    // Call the divisible_sum_pairs function and write the result to a file
    let result = divisible_sum_pairs(n, k, &ar);
    writeln!(&mut fptr, "{}", result).ok();
}
