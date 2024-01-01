use std::{io::{BufReader, BufRead}, fs::File, path::Path};

fn main() {
    let input = std::fs::read_to_string("example-puzzle-input.txt").unwrap();
}

fn solve_part_1(input_file_name: &str) -> i32 {
    0
}

#[test]
fn test_solve_part_1() {
    assert_eq!(solve_part_1("example-puzzle-input.txt"), 136)
}

fn read_input_file(input_file_name: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(input_file_name).expect("no such file");
    let buf = BufReader::new(file);
    let rows: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    rows
}

#[test]
fn test_read_input_file() {
    let rows: Vec<String> = read_input_file("example-puzzle-input.txt");
    assert_eq!(rows[0], "O....#....");    // Assert that first line is the expected.
    assert_eq!(rows[9], "#OO..#....");    // Assert that last line is the expected. 
    assert_eq!(rows.len(), 10);           // Assert that number of lines is the expected.
}
