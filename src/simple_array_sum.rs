use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

// Function 'simple_array_sum' takes an array of integers and returns their sum
fn simple_array_sum(ar: &[i32]) -> i32 {
    ar.iter().sum()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // Read the number of elements in the array
    let _ar_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Read the array of integers
    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    // Call the simple_array_sum function and write the result to a file
    let result = simple_array_sum(&ar);
    writeln!(&mut fptr, "{}", result).ok();
}