//! # Counter
//!
//! Accepts numeric input, then counts the number of occurances of
//! each number.

use std::collections::HashMap;
use std::io::{self, Write};
use itertools::Itertools;

/// Prints a message, then gets a line of input.
///
/// # Errors
/// Returns `io::error` if it fails to read from `stdin`.
///
/// # Examples
/// ```no_run
/// loop {
///     let num = input("Enter a number:");
///     // do something with num
/// }
/// ```
fn input(message: &str) -> Result<String, io::Error>
{
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut ret = String::new();
    io::stdin().read_line(&mut ret)?;
    Ok(ret) // propagate errors (because we used the ? operator)
    // Result(ret)
}

fn get_numbers() -> Vec<i32> {
    let mut result = Vec::new();
    loop {
        let line = input("Enter a number (blank to stop): ").expect("Failed to read input");
        match line.trim().len() {
            0 => break,
            _ => {
                result.push(line.trim().parse::<i32>().expect("Could not get integer"));
            }
        }
    }
    result
}

fn counter(numbers: &Vec<i32>) -> HashMap<i32,usize> {
    let mut result = HashMap::new();
    for num in numbers {
        // let count = result.insert(&num, 0).unwrap();
        let count = result.entry(*num).or_insert(0);
        *count += 1;
    }
    result
}


fn main() {
    let numbers = get_numbers();
    let count = counter(&numbers);
    println!("\nCounts:");
    for num in count.keys().sorted() {
        println!("{}: {}", num, count[num]);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     // write some tests here
// }

