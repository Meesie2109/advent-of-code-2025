use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut dail = 50;
    let mut count = 0;

    let file = File::open("day-1/part-1/input.txt")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("Input line: {}", line);

        if line.starts_with("R") {
            let number: i32 = line[1..].parse()?;
            dail = (dail + number) % 100;
        } else if line.starts_with("L") {
            let number: i32 = line[1..].parse()?;
            dail = (dail - number + 100) % 100;
        }

        if dail == 0 {
            count += 1;
        }

        println!("Dial = {}, Count = {}", dail, count);
    }

    println!("Final Count {}", count);
    Ok(())
}
