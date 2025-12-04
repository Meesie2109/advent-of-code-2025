use advent_of_code_2025::days;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let day = 1;
    let input = std::fs::read_to_string(format!("src/inputs/day{}.txt", day))?;

    println!("Day 1, Part 1: {}", days::day1::solve_part1(&input)?);
    println!("Day 1, Part 2: {}", days::day1::solve_part2(&input)?);

    Ok(())
}
