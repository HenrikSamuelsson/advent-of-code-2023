use std::{collections::HashMap, fs::File, io::BufRead, io::BufReader, path::Path};

fn main() {
    solve_part_1("example-puzzle-input.txt");
}

pub fn solve_part_1(input_file_name: &str) -> u32 {
    let schematic = map_schematic(input_file_name);
    for row in (0..10).rev() {
        for col in (0..10).rev() {
            match schematic.get(&(col, row)) {
                Some(&c) => print!("{}", c),
                _ => print!("?"),
            }
        }
        println!("");
    }
    0
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
