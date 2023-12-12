use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

const EXAMPLE_PUZZLE_INPUT_FILE: &str = "example-puzzle-input.txt";

fn main() {
     // Create a path to the desired file
     let path = Path::new(EXAMPLE_PUZZLE_INPUT_FILE);
     let display = path.display();

     // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, <dyn Error>::to_string(&why)),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();

    for line in lines {
        println!("{:?}", line);
    }
}

pub fn solve_part_1(input_text: &str) -> u32 {
    0
}

#[test]
    fn test_solve_part_1() {
        assert_eq!(solve_part_1(&EXAMPLE_PUZZLE_INPUT_FILE), 8)
}