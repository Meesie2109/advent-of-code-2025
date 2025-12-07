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

pub fn solve_part2(input: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let mut ranges: Vec<(i64, i64)> = input
        .split("\n\n")
        .next()
        .unwrap_or("")
        .lines()
        .map(|line| split_range(line))
        .collect::<Result<_, _>>()?;

    ranges.sort_by_key(|&(start, _)| start);

    let mut merged = Vec::new();
    if let Some((mut current_start, mut current_end)) = ranges.first().cloned() {
        for &(start, end) in &ranges[1..] {
            if start <= current_end {
                current_end = current_end.max(end);
            } else {
                merged.push((current_start, current_end));
                current_start = start;
                current_end = end;
            }
        }
        merged.push((current_start, current_end));
    }

    let mut result = 0;
    for &(start, end) in &merged {
        result += (end - start) + 1;
    }

    Ok(result)
}

pub fn split_range(range: &str) -> Result<(i64, i64), Box<dyn std::error::Error>> {
    let (start, end) = range.split_once("-").ok_or("No `-` in range")?;

    let start_num = start.parse::<i64>()?;
    let end_num = end.parse::<i64>()?;

    Ok((start_num, end_num))
}
