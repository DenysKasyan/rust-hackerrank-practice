use std::io::{self, BufRead};

/*
 * Complete the 'bon_appetit' function below.
 *
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY bill
 *  2. INTEGER k
 *  3. INTEGER b
 */
fn bon_appetit(bill: &[i32], k: i32, b: i32) {
    let actual_share = (bill.iter().sum::<i32>() - bill[k as usize]) / 2;
    if b == actual_share {
        println!("Bon Appetit");
    } else {
        println!("{}", b - actual_share);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // Read the first multiple input
    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();
    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    // Read the array of bill items
    let bill: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    // Read the amount charged
    let b = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Call the bon_appetit function
    bon_appetit(&bill, k, b);
}
