use std::{collections::{HashMap, HashSet}, fs::File, io::BufRead, io::BufReader, path::Path};

/// Type used to track position of numbers and symbols on the engine schematic.
/// 
/// x grows horizontally from left to right.
/// y grows vertically from top to bottom.
struct Coordinate {
    x: i32,
    y: i32,
}

struct Schematic {
    rows: usize,
    cols: usize,
    map: HashMap<Coordinate, char>,
}

fn main() {
    solve_part_1("example-puzzle-input.txt");
}

pub fn solve_part_1(input_file_name: &str) -> u32 {
    // Read the complete engine schematic from file into a map.
    let schematic = map_schematic(input_file_name);
    let schematic_strings = vec_schematic(input_file_name);
    let schematic_chars = vec!['.'; schematic.keys().len()];

    // Vector for holding all the symbol locations of the engine schematic.
    let mut schematic_symbol_locations: Vec<(i32, i32)> = vec![];
    // Loop to search the for symbols in the schematic.
    for (y, line) in schematic_strings.iter().enumerate() {
        println!("{:?}", line);
        for (x, c) in line.chars().enumerate() {
            if !c.is_numeric() {
                if c != '.' {
                    let x: i32 = x.try_into().unwrap();
                    let y: i32 = y.try_into().unwrap();
                    schematic_symbol_locations.push((x, y));
                    println!("{} {} {}", c, x, y);
                }
            }
        }
    }

    0
}

fn map_schematic(filename: impl AsRef<Path>) -> HashMap<(i32, i32), char> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    // Collect each input line into a vector.
    let schematic: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    // Map engine parts based on schematics coordinates.
    let mut engine_parts_map: HashMap<(i32, i32), char> = [].into();
    for (y, line) in schematic.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            engine_parts_map.insert((x.try_into().unwrap(), y.try_into().unwrap()), c);
        }
    }


    /*
    // Loop to search the for symbols in the schematic.
    for (y, line) in schematic.iter().enumerate() {
        println!("{:?}", line);
        for (x, c) in line.chars().enumerate() {
            if !c.is_numeric() {
                if c != '.' {
                    // let x = x.try_into().unwrap();
                    //let y = y.try_into().unwrap();
                    //schematic_symbol_locations.push((x, y));
                    println!("{} {} {}", c, x, y);
                }
            }
        }
    }
    */

    engine_parts_map
}

fn vec_schematic(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);

    // Collect each input line into a vector.
    let schematic: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    
    schematic
}

#[test]
fn test_solve_part_1() {
    assert_eq!(solve_part_1(&"example-puzzle-input.txt"), 4361)
}
