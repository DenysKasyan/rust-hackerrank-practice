use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'sock_merchant' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER_ARRAY ar
 */
fn sock_merchant(_n: i32, ar: &[i32]) -> i32 {
    let mut sock_counts = std::collections::HashMap::new();

    for &sock in ar {
        *sock_counts.entry(sock).or_insert(0) += 1;
    }

    sock_counts.values().map(|&count| count / 2).sum()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // Read the number of socks
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Read the array of socks
    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    // Call the sock_merchant function and write the result to a file
    let result = sock_merchant(n, &ar);
    writeln!(&mut fptr, "{}", result).ok();
}
