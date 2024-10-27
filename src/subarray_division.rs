use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'birthday' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY s
 *  2. INTEGER d
 *  3. INTEGER m
 */
fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    let mut count = 0;
    for i in 0..=(s.len() - m as usize) {
        if s[i..i + m as usize].iter().sum::<i32>() == d {
            count += 1;
        }
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // Read the number of integers
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Read the array of integers
    let s: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    // Read the values for d and m
    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();
    let d = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    // Call the birthday function and write the result to a file
    let result = birthday(&s, d, m);
    writeln!(&mut fptr, "{}", result).ok();
}
