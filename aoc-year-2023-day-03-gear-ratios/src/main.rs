use std::{collections::HashMap, fs::File, io::BufRead, io::BufReader, path::Path};

fn main() {
    solve_part_1("example-puzzle-input.txt");
}

pub fn solve_part_1(input_file_name: &str) -> u32 {
    let result: u32 = 0;
    let schematic = map_schematic(input_file_name);
    // Loop through the schematic backwards.
    for row in (0..10).rev() {
        let mut char_part_of_number: bool = false;
        let mut current_number = 0;
        let mut multiplier = 1;
        let mut first_col_of_number;
        let mut last_col_of_number = 0;
        for col in (0..10).rev() {
            match schematic.get(&(col, row)) {
                Some(&c) => {
                    // If finding an ascii digit this shall be parsed into corresponding integer.
                    if c.is_ascii_digit() {
                        if !char_part_of_number {
                            // Found first digit in an engine part number.
                            char_part_of_number = true;
                            last_col_of_number = col;
                            multiplier = 1;
                            current_number = c.to_digit(10).expect("Not a number");
                        } else {
                            // Found another digit in an engine part number.
                            multiplier *= 10;
                            current_number += multiplier * c.to_digit(10).expect("Not a number");
                        }
                    } else {
                        if char_part_of_number {
                            // Have moved beyond the last digit in current engine part number.
                            char_part_of_number = false;
                            first_col_of_number = col + 1;
                            println!(
                                "{} {} {}",
                                current_number, first_col_of_number, last_col_of_number
                            );
                                if is_number_adjacent_to_symbol(
                                    row,
                                    first_col_of_number,
                                    last_col_of_number,
                                    schematic.clone(),
                                ) {
                                    println!("Adjacent to symbol");
                                } else {
                                    println!("Not adjacent to symbol.");
                                }
                        }
                    }
                }
                _ => (),
            }
        }
        if char_part_of_number {
            // Special case that a number is at the end of a line
            first_col_of_number = 0;
            println!(
                "{} {} {}",
                current_number, first_col_of_number, last_col_of_number
            );
            if is_number_adjacent_to_symbol(
                row,
                first_col_of_number,
                last_col_of_number,
                schematic.clone(),
            ) {
                println!("Adjacent to symbol");
            } else {
                println!("Not adjacent to symbol.");
            }
        }
    }
    result
}
/// Looks at the frame around a section in the schematics for adjacent symbols.
fn is_number_adjacent_to_symbol(
    row: i32,
    first_col: i32,
    last_col: i32,
    s: HashMap<(i32, i32), char>,
) -> bool {
    let mut result = false;
    for col in (first_col - 1)..=(last_col + 1) {
        // Check the row above.
        match s.get(&(col, row - 1)) {
            Some(&c) => {
                if c != '.' && !c.is_ascii_digit() {
                    result = true
                }
            }
            _ => (),
        }
        // Check the row below.
        match s.get(&(col, row + 1)) {
            Some(&c) => {
                if c != '.' && !c.is_ascii_digit() {
                    result = true
                }
            }
            _ => (),
        }
        // Check to the left.
        match s.get(&(first_col - 1, row)) {
            Some(&c) => {
                if c != '.' && !c.is_ascii_digit() {
                    result = true
                }
            }
            _ => (),
        }
        // Check to the right.
        match s.get(&(last_col + 1, row)) {
            Some(&c) => {
                if c != '.' && !c.is_ascii_digit() {
                    result = true
                }
            }
            _ => (),
        }
    }
    result
}

/// Read the complete engine schematic from file into a map.
fn map_schematic(filename: impl AsRef<Path>) -> HashMap<(i32, i32), char> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    // Collect each input line into a vector.
    let schematic: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    // Map engine parts based on schematics coordinates.
    let mut engine_parts_map: HashMap<(i32, i32), char> = HashMap::new();
    for (y, line) in schematic.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            engine_parts_map.insert((x.try_into().unwrap(), y.try_into().unwrap()), c);
        }
    }
    engine_parts_map
}

#[test]
fn test_solve_part_1() {
    assert_eq!(solve_part_1(&"example-puzzle-input.txt"), 4361)
}
