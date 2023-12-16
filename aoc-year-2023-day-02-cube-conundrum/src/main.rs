use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

/// Name of file with example input given in the puzzle description.
const EXAMPLE_PUZZLE_INPUT_FILE: &str = "example-puzzle-input.txt";

/// Name of file holding personal puzzle input.
const PERSONAL_PUZZLE_INPUT_FILE: &str = "personal-puzzle-input.txt";

/// Number of available cubes by colour according to puzzle description.
const RED_CUBE_COUNT: u32 = 12;
const GREEN_CUBE_COUNT: u32 = 13;
const BLUE_CUBE_COUNT: u32 = 14;

fn main() {
    let result_part_1 = solve_part_1(PERSONAL_PUZZLE_INPUT_FILE);
    println!("Result part 1 = {}", result_part_1);
    
    let result_part_2 = solve_part_2(PERSONAL_PUZZLE_INPUT_FILE);
    println!("Result part 2 = {}", result_part_2);
}

pub fn solve_part_1(input_file_name: &str) -> u32 {
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

    for line in lines {
        let mut valid_cube_count = true;
        let mut game_id_number: u32 = 0;
        if let Ok(game_record) = line {
            // Split the game record into two parts.
            let mut game_record_parts = game_record.split(":");

            // Parse the game id number in the first game record part.
            let game_id = game_record_parts.next();
            let mut game_id_parts = game_id.expect("GAME ID").split(" ");
            assert_eq!(Some("Game"), game_id_parts.next());
            game_id_number = game_id_parts.next().expect("ID NUMBER").parse().unwrap();

            // Start checking the cube counts in the second game record part.

            let cube_counts = game_record_parts.next();
            // Split on semicolon gives a set of cubes, possibly in different colours.
            let cube_sets = cube_counts.expect("CUBE SET").split(";");
            for cube_set in cube_sets {
                // Split on comma gives a cube count in a single colour, for example "5 green".
                let cubes_by_colour = cube_set.split(",");
                for cubes_in_one_colour in cubes_by_colour {
                    // Split on space to separate the cube count from the colour.
                    let mut cube_count = cubes_in_one_colour.split(" ");
                    cube_count.next(); // Hack to get rid of initial empty part.
                    let num_of_cubes: u32 = cube_count.next().expect("CUBE COUNT").parse().unwrap();
                    print!("{} ", num_of_cubes);
                    let cube_colour = cube_count.next();
                    if let Some(colour) = cube_colour {
                        println!("{:?}", colour);
                        match colour {
                            "blue" => {
                                if num_of_cubes > BLUE_CUBE_COUNT {
                                    valid_cube_count = false
                                }
                            }
                            "green" => {
                                if num_of_cubes > GREEN_CUBE_COUNT {
                                    valid_cube_count = false
                                }
                            }
                            "red" => {
                                if num_of_cubes > RED_CUBE_COUNT {
                                    valid_cube_count = false
                                }
                            }
                            _ => println!("Unexpected colour",),
                        }
                    }
                }
                println!("");
            }
        }
        if valid_cube_count {
            result += game_id_number;
        }
    }
    result
}

pub fn solve_part_2(input_file_name: &str) -> u32 {
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

    for line in lines {
        let mut game_id_number: u32 = 0;
        let mut blue_max: u32 = 0;
        let mut green_max: u32 = 0;
        let mut red_max: u32 = 0;
        if let Ok(game_record) = line {
            // Split the game record into two parts.
            let mut game_record_parts = game_record.split(":");

            // Parse the game id number in the first game record part.
            let game_id = game_record_parts.next();
            let mut game_id_parts = game_id.expect("GAME ID").split(" ");
            assert_eq!(Some("Game"), game_id_parts.next());
            game_id_number = game_id_parts.next().expect("ID NUMBER").parse().unwrap();

            // Start checking the cube counts in the second game record part.
            let cube_counts = game_record_parts.next();
            // Split on semicolon gives a set of cubes, possibly in different colours.
            let cube_sets = cube_counts.expect("CUBE SET").split(";");
            for cube_set in cube_sets {
                // Split on comma gives a cube count in a single colour, for example "5 green".
                let cubes_by_colour = cube_set.split(",");
                for cubes_in_one_colour in cubes_by_colour {
                    // Split on space to separate the cube count from the colour.
                    let mut cube_count = cubes_in_one_colour.split(" ");
                    cube_count.next(); // Hack to get rid of initial empty part.
                    let num_of_cubes: u32 = cube_count.next().expect("CUBE COUNT").parse().unwrap();
                    print!("{} ", num_of_cubes);
                    let cube_colour = cube_count.next();
                    if let Some(colour) = cube_colour {
                        println!("{:?}", colour);
                        match colour {
                            "blue" => {
                                if num_of_cubes > blue_max {
                                    blue_max = num_of_cubes;
                                }
                            }
                            "green" => {
                                if num_of_cubes > green_max {
                                    green_max = num_of_cubes;
                                }
                            }
                            "red" => {
                                if num_of_cubes > red_max {
                                    red_max = num_of_cubes;
                                }
                            }
                            _ => println!("Unexpected colour"),
                        }
                    }
                }
                println!("");
            }
        }
        result += blue_max * green_max * red_max;
    }
    result
}


#[test]
fn test_solve_part_1() {
    assert_eq!(solve_part_1(&EXAMPLE_PUZZLE_INPUT_FILE), 8)
}

#[test]
fn test_solve_part_2() {
    assert_eq!(solve_part_2(&EXAMPLE_PUZZLE_INPUT_FILE), 2286)
}
