use std::io::{self, BufRead};

/*
 * Complete the 'count_apples_and_oranges' function below.
 *
 * The function accepts following parameters:
 *  1. INTEGER s
 *  2. INTEGER t
 *  3. INTEGER a
 *  4. INTEGER b
 *  5. INTEGER_ARRAY apples
 *  6. INTEGER_ARRAY oranges
 */
fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apple_count = apples.iter().filter(|&&d| a + d >= s && a + d <= t).count();
    let orange_count = oranges.iter().filter(|&&d| b + d >= s && b + d <= t).count();

    println!("{}", apple_count);
    println!("{}", orange_count);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Read the first multiple input
    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();
    let s = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let t = first_multiple_input[1].trim().parse::<i32>().unwrap();

    // Read the second multiple input
    let second_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();
    let a = second_multiple_input[0].trim().parse::<i32>().unwrap();
    let b = second_multiple_input[1].trim().parse::<i32>().unwrap();

    // Read the third multiple input
    let third_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();
    let m = third_multiple_input[0].trim().parse::<i32>().unwrap();
    let n = third_multiple_input[1].trim().parse::<i32>().unwrap();

    // Read the apples distances
    let apples: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    // Read the oranges distances
    let oranges: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    count_apples_and_oranges(s, t, a, b, &apples, &oranges);
}
