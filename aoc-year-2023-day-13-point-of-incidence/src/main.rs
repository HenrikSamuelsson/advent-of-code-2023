use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn main() {
    println!("Hello, world!");
}

/// Reads puzzle input from a file into a vector of strings.
/// Each string in the vector represents one line of input.
fn read_input_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    // Collect each input line into a vector.
    let rows: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    rows
}

#[test]
fn test_read_input_file() {
    let rows: Vec<String> = read_input_file("example-puzzle-input.txt");
    assert_eq!(rows[0], "#.##..##.");
    assert_eq!(rows[1], "..#.##.#.");
    assert_eq!(rows[2], "##......#");
    assert_eq!(rows[3], "##......#");
    assert_eq!(rows[4], "..#.##.#.");
    assert_eq!(rows[5], "..##..##.");
    assert_eq!(rows[6], "#.#.##.#.");
    assert_eq!(rows[7], "");
    assert_eq!(rows[8], "#...##..#");
    assert_eq!(rows[9], "#....#..#");
    assert_eq!(rows[10], "..##..###");
    assert_eq!(rows[11], "#####.##.");
    assert_eq!(rows[12], "#####.##.");
    assert_eq!(rows[13], "..##..###");
    assert_eq!(rows[14], "#....#..#");
    assert_eq!(rows.len(), 15);
}
