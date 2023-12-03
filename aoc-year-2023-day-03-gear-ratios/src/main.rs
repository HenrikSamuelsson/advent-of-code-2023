use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let y_min = 0;
    let x_min = 0;
    let y_max: usize;
    let x_max: usize;

    // File must exist in the current path.
    if let Ok(lines) = read_lines("./example-puzzle-input.txt") {
        println!("Read puzzle example input.");
        // Find out the size of the engine schematics
        y_max = lines.count() - 1;
        println!("y_max = {}", y_max);
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
