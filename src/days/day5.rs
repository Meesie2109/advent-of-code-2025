pub fn solve_part1(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut parts = input.split("\n\n");
    let ranges: Vec<&str> = parts.next().unwrap_or("").lines().collect();
    let numbers: Vec<i64> = parts
        .next()
        .unwrap_or("")
        .lines()
        .filter_map(|l| l.parse().ok())
        .collect();

    let mut result = 0;

    'num_loop: for num in numbers {
        for range in &ranges {
            let (start, end) = range.split_once("-").ok_or("No `-` in range")?;

            let start_num = start.parse::<i64>()?;
            let end_num = end.parse::<i64>()?;

            if num >= start_num && num <= end_num {
                result += 1;
                continue 'num_loop;
            }
        }
    }

    Ok(result)
}
