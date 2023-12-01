use std::fs::File;

fn main() -> Result<(), std::io::Error> {
    let file = File::open("aoc-2023-day-01-input.txt")?;
    print!("{:?}",file);
    Ok(())
}
