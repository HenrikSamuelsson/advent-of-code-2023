
fn main() {
}

pub fn solve_part_1(_input_file_name: &str) -> u32 {
    // Create a path to the desired file
    let path = Path::new(input_file_name);

    // Open the path in read-only mode.
    let file = match File::open(&path) {
        Err(why) => panic!(
            "Could not open input file: {}",
            <dyn Error>::to_string(&why)
        ),
        Ok(file) => file,
    };

    let mut result: u32 = 0;

    let reader = BufReader::new(file);
    let lines = reader.lines();
    0
}

#[test]
fn test_solve_part_1() {
    assert_eq!(solve_part_1(&"example-puzzle-input.txt"), 4361)
}
