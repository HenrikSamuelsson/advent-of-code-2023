use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File aoc-2023-day-01-input.txt must exist in the current path.
    if let Ok(lines) = read_lines("./aoc-2023-day-01-input.txt") {
        // Consumes the iterator, returns an (Optional) String.
        for line in lines {
            if let Ok(amended_line) = line {
                println!("{}", amended_line);
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
