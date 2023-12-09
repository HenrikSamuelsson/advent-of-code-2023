use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File must exist in the current path.
    if let Ok(lines) = read_lines("./example-puzzle-input.txt") {
        for line in lines {
            if let Ok(scratch_card) = line {
                println!("{}", scratch_card);
                let cropped = crop_letters(&scratch_card, 7);
                println!("{cropped}");
            }
        }
    } else {
        println!("Error reading lines from file.");
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}