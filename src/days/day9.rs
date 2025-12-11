pub fn solve_part1(input: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let lines: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num_str| num_str.parse::<i64>().unwrap_or(0))
                .collect::<Vec<i64>>()
        })
        .collect();

    let mut larges = 0;
    for (i, start) in lines.iter().enumerate() {
        for (j, end) in lines.iter().enumerate() {
            if i == j {
                continue;
            }

            let dx = (end[1] - start[1]).abs() + 1;
            let dy = (end[0] - start[0]).abs() + 1;
            let area = dx * dy;

            if area > larges {
                larges = area;
            }
        }
    }

    Ok(larges)
}
