use advent_of_code_2025::days;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for day in 1..=8 {
        let input = fs::read_to_string(format!("src/inputs/day{}.txt", day))?;
        match day {
            1 => {
                println!("Day 1, Part 1: {}", days::day1::solve_part1(&input)?);
                println!("Day 1, Part 2: {}", days::day1::solve_part2(&input)?);
            }
            2 => {
                println!("Day 2, Part 1: {}", days::day2::solve_part1(&input)?);
                println!("Day 2, Part 2: {}", days::day2::solve_part2(&input)?);
            }
            3 => {
                println!("Day 3, Part 1: {}", days::day3::solve_part1(&input)?);
                println!("Day 3, Part 1: {}", days::day3::solve_part2(&input)?);
            }
            4 => {
                println!("Day 4, Part 1: {}", days::day4::solve_part1(&input)?);
                println!("Day 4, Part 2: {}", days::day4::solve_part2(&input)?);
            }
            5 => {
                println!("Day 5, Part 1: {}", days::day5::solve_part1(&input)?);
                println!("Day 5, Part 2: {}", days::day5::solve_part2(&input)?);
            }
            6 => {
                println!("Day 6, Part 1: {}", days::day6::solve_part1(&input)?);
                println!("Day 6, Part 2: {}", days::day6::solve_part2(&input)?);
            }
            7 => {
                println!("Day 7, Part 1: {}", days::day7::solve_part1(&input)?);
                println!("Day 7, Part 2: {}", days::day7::solve_part2(&input)?);
            }
            8 => {
                println!("Day 8, Part 1: {}", days::day8::solve_part1(&input)?);
            }
            _ => {}
        }
    }

    Ok(())
}
