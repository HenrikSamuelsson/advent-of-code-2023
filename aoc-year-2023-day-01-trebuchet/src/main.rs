use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File must exist in the current path.
    if let Ok(lines) = read_lines("./example-puzzle-input.txt") {
        let mut sum_part_1 = 0;
        for line in lines {
            let mut first_digit = 0;
            let mut last_digit = 0;
            if let Ok(ref amended_line) = line {
                println!("{}", amended_line);
                if let Some(digit) = find_first_digit(&amended_line) {
                    first_digit = digit;
                }
                if let Some(digit) = find_last_digit(&amended_line) {
                    last_digit = digit;
                }

                println!("first_digit = {}", first_digit);
                println!("last_digit = {}", last_digit);

                sum_part_1 += 10 * first_digit + last_digit;
            }
        }
        println!("Answer part 1 = {}", sum_part_1);
    } else {
        println!("Error reading lines from file.");
    }
    

    // Part 2
    let mut sum_part_2 = 0;
    if let Ok(lines2) = read_lines("./example-puzzle-input2.txt") {
        for line in lines2 {
            let mut first_digit = 0;
            let mut last_digit = 0;
            if let Ok(ref amended_line2) = line {
                println!("{}", amended_line2);
                let mut chars = amended_line2.chars();
                loop {
                    if let Some(digit) = parse_number_at_start(&chars.as_str()) {
                        first_digit = digit;
                        println!("first_digit parsed {}", first_digit);
                        break;
                    }
                    if let Some(ch) = chars.next() {
                        if let Some(digit) = ch.to_digit(10) {
                            first_digit = digit;
                            println!("first_digit single char {}", first_digit);
                            break;
                        }
                    } else {
                        break;
                    }
                }

                chars = amended_line2.chars();
                loop {
                    if let Some(digit) = parse_number_at_end(&chars.as_str()) {
                        last_digit = digit;
                        println!("first_digit parsed {}", first_digit);
                        break;
                    }
                    if let Some(ch) = chars.next_back() {
                        if let Some(digit) = ch.to_digit(10) {
                            last_digit = digit;
                            println!("first_digit single char {}", first_digit);
                            break;
                        }
                    } else {
                        break;
                    }
                }

                sum_part_2 += 10 * first_digit + last_digit;
            }
        }
    } else {
        println!("Error reading lines from file.");
    }
    println!("Answer part 2 = {}", sum_part_2);
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Finds the first digit in a given string.
fn find_first_digit(text: &str) -> Option <u32> {
    for c in text.chars() {
        if c.is_numeric() {
            return Some(c.to_digit(10).unwrap());
        }
    }
    None
}

/// Finds the last digit in a given string.
fn find_last_digit(text: &str) -> Option<u32> {
    for c in text.chars().rev() {
        if c.is_numeric() {
            return Some(c.to_digit(10).unwrap());
        }
    }
    None
}

/// Finds letter combinations corresponding to one digit numbers at the start of a given string.
fn parse_number_at_start(text: &str) -> Option<u32> {
    const NUMBERS: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for (i, n) in NUMBERS.iter().enumerate() {
        if text.starts_with(n) {
            return Some(i as u32);
        }
    }
    None
}

/// Finds letter combinations corresponding to one digit numbers at the end of a given string.
fn parse_number_at_end(text: &str) -> Option<u32> {
    const NUMBERS: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for (i, n) in NUMBERS.iter().enumerate() {
        if text.ends_with(n) {
            return Some(i as u32);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::parse_number_at_start;
    use crate::parse_number_at_end;

    const LINE_TWO1NINE: &str = "two1nine";
    const LINE_1TWO1: &str = "1two1";

    #[test]
    fn parse_number_test() {
        assert_eq!(parse_number_at_start(&LINE_TWO1NINE), Some(2));
        assert_eq!(parse_number_at_end(&LINE_TWO1NINE), Some(9));
        assert_eq!(parse_number_at_start(&LINE_1TWO1), None);
        assert_eq!(parse_number_at_end(&LINE_1TWO1), None);
    }
}
