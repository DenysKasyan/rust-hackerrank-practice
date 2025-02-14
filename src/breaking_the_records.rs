use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'breaking_records' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY scores as parameter.
 */
fn breaking_records(scores: &[i32]) -> Vec<i32> {
    let mut min_score = scores[0];
    let mut max_score = scores[0];
    let mut min_breaks = 0;
    let mut max_breaks = 0;

    for &score in scores.iter().skip(1) {
        if score < min_score {
            min_score = score;
            min_breaks += 1;
        }
        if score > max_score {
            max_score = score;
            max_breaks += 1;
        }
    }

    vec![max_breaks, min_breaks]
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // Read the number of scores
    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Read the scores into a vector
    let scores: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    // Call the breaking_records function and write the result to a file
    let result = breaking_records(&scores);
    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();
        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }
    writeln!(&mut fptr).ok();
}
