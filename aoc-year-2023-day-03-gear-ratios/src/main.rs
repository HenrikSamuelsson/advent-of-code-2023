use std::{io::{BufRead, Lines, Read}, io::BufReader, path::{Path, PathBuf}, fs::File, error::Error};



fn main() {
    solve_part_1("example-puzzle-input.txt");
}

pub fn solve_part_1(input_file_name: &str) -> u32 {
    // Create a path to the desired file
    let lines = lines_from_file("example-puzzle-input.txt");   
    for line in lines {
        println!("{:?}", line);
    }
    0
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[test]
fn test_solve_part_1() {
    assert_eq!(solve_part_1(&"example-puzzle-input.txt"), 4361)
}
