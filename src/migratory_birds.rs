use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'migratory_birds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */
fn migratory_birds(arr: &[i32]) -> i32 {
    let mut frequency = vec![0; 6];

    for &bird in arr {
        frequency[bird as usize] += 1;
    }

    let mut max_frequency = 0;
    let mut bird_type_with_max_freq = 0;

    for (bird_type, &freq) in frequency.iter().enumerate().skip(1) {
        if freq > max_frequency {
            max_frequency = freq;
            bird_type_with_max_freq = bird_type as i32;
        } else if freq == max_frequency && bird_type_with_max_freq > bird_type as i32 {
            bird_type_with_max_freq = bird_type as i32;
        }
    }

    bird_type_with_max_freq
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // Read the number of elements
    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Read the array of bird IDs
    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    // Call the migratory_birds function and write the result to a file
    let result = migratory_birds(&arr);
    writeln!(&mut fptr, "{}", result).ok();
}
