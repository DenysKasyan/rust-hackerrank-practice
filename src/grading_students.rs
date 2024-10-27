use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'grading_students' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY grades as parameter.
 */
fn grading_students(grades: &[i32]) -> Vec<i32> {
    let mut rounded_grades = Vec::with_capacity(grades.len());
    for &grade in grades {
        if grade >= 38 {
            let next_multiple_of_5 = ((grade / 5) + 1) * 5;
            if next_multiple_of_5 - grade < 3 {
                rounded_grades.push(next_multiple_of_5);
            } else {
                rounded_grades.push(grade);
            }
        } else {
            rounded_grades.push(grade);
        }
    }
    rounded_grades
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    // Read the number of grades
    let grades_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    // Read the grades into a vector
    let mut grades: Vec<i32> = Vec::with_capacity(grades_count as usize);
    for _ in 0..grades_count {
        let grades_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        grades.push(grades_item);
    }

    // Call the grading_students function and write the result to a file
    let result = grading_students(&grades);
    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();
        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }
    writeln!(&mut fptr).ok();
}
