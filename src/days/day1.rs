pub fn solve_part1(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut dail = 50;
    let mut count = 0;

    for line in input.lines() {
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
    }

    Ok(count)
}

pub fn solve_part2(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut dail = 50;
    let mut count = 0;

    for line in input.lines() {
        let number: i32 = line[1..].parse()?;

        if line.starts_with("R") {
            count += (dail + number) / 100;
            dail = (dail + number) % 100;
        } else if line.starts_with("L") {
            let reversed = (100 - dail) % 100;
            count += (reversed + number) / 100;

            dail = (dail - number).rem_euclid(100);
        }
    }

    Ok(count)
}
