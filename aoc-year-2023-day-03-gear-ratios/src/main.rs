use std::{io::BufRead, io::BufReader, path::Path, fs::File};

fn main() {
    solve_part_1("example-puzzle-input.txt");
}

pub fn solve_part_1(input_file_name: &str) -> u32 {
    // Read the complete engine schematic from file.
    let schematic = schematic_from_file(input_file_name);
    
    // Vector for holding all the symbol locations of the engine schematic.
    let mut schematic_symbol_locations: Vec<(u32, u32)> = vec![];
    // Loop to search the for symbols in the schematic.
    for (y, line) in schematic.iter().enumerate() {
        println!("{:?}", line);
        for (x, c) in line.chars().enumerate() {
            if !c.is_numeric() {
                if c != '.' {
                    let x = x.try_into().unwrap();
                    let y = y.try_into().unwrap();
                    schematic_symbol_locations.push((x, y));
                    println!("{} {} {}", c, x, y);
                }
            }
        }
    }

    // Vector for holding all the numbers adjacent to a symbol.
    let mut schematic_symbol_locations: Vec<(u32, u32)> = vec![];
    // Loop to search for any number adjacent to a symbol.
    0
}

fn schematic_from_file(filename: impl AsRef<Path>) -> Vec<String> {
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
