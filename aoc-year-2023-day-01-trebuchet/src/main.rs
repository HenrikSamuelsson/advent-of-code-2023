use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File must exist in the current path.
    if let Ok(lines) = read_lines("./example-puzzle-input.txt") {
        // Consumes the iterator, returns an (Optional) String.
        let mut sum = 0;
        for line in lines {
            let first_digit;
            let last_digit;
            if let Ok(amended_line) = line {
                println!("{}", amended_line);
                first_digit = find_first_digit(&amended_line);
                last_digit = find_last_digit(&amended_line);
 
                println!("first_digit = {}", first_digit);
                println!("last_digit = {}", last_digit);
                
                sum += 10 * first_digit + last_digit;
            }
        }
        println!("Answer part 1 = {}", sum);
    }
    else {
        println!("Error reading lines from file.");
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Finds the first digit in a given string.
/// Note, will return 0 if no digit is found in the string.
fn find_first_digit(text: &str) -> u32 {
    for c in text.chars() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap();
        }
    }
    0
}

/// Finds the last digit in a given string.
/// Note, will return 0 if no digit is found in the string.
fn find_last_digit(text: &str) -> u32 {
    for c in text.chars().rev() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap();
        }
    }
    0
}
