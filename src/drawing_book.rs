use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'page_count' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER p
 */
fn page_count(n: i32, p: i32) -> i32 {
    let total_turns_from_front = p / 2;
    let total_turns_from_back = (n / 2) - (p / 2);
    total_turns_from_front.min(total_turns_from_back)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // Read the total number of pages
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Read the page number to turn to
    let p = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Call the page_count function and write the result to a file
    let result = page_count(n, p);
    writeln!(&mut fptr, "{}", result).ok();
}
