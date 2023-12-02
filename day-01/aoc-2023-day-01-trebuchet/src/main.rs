use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File aoc-2023-day-01-example-input.txt must exist in the current path.
    if let Ok(lines) = read_lines("./aoc-2023-day-01-example-input.txt") {
        // Consumes the iterator, returns an (Optional) String.
        let mut sum = 0;
        for line in lines {
            let mut first_digit = 0;
            let mut last_digit = 0;
            let mut missing_first_digit = true;
            if let Ok(amended_line) = line {
                println!("{}", amended_line);
                for c in amended_line.chars() {
                    if c.is_numeric() {
                        if missing_first_digit {
                            first_digit = c.to_digit(10).unwrap();
                            missing_first_digit = false;
                        }
                        last_digit = c.to_digit(10).unwrap();
                    }
                }
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
