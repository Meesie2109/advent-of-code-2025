pub fn solve_part1(input: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let lines: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let mut positions: Vec<usize> = Vec::new();
    let mut count = 0;
    for (i, line) in lines.iter().enumerate() {
        if i == 0 {
            let start_pos = line.iter().position(|&c| c == 'S').unwrap_or(0);
            positions.push(start_pos);
        } else {
            let mut new_positions: Vec<usize> = Vec::new();
            for p in &positions {
                println!("found ={:#?}", line[*p] == '^');

                if line[*p] == '^' {
                    count += 1;
                    if !new_positions.contains(&(p - 1)) {
                        new_positions.push(p - 1);
                    }
                    if !new_positions.contains(&(p + 1)) {
                        new_positions.push(p + 1);
                    }
                } else {
                    if !new_positions.contains(&(p)) {
                        new_positions.push(*p);
                    }
                }
            }

            positions = new_positions;
        }
    }

    Ok(count)
}

pub fn solve_part2(input: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let width = lines[0].len();
    let mut counts = vec![0i64; width];

    let start_pos = lines[0].iter().position(|&c| c == 'S').unwrap_or(0);
    counts[start_pos] = 1;

    for line in lines.iter().skip(1) {
        let mut new_counts = vec![0i64; width];

        for col in 0..width {
            let c = line[col];
            let current = counts[col];
            if current == 0 {
                continue;
            }

            if c == '^' {
                if col > 0 {
                    new_counts[col - 1] += current;
                }
                if col < width - 1 {
                    new_counts[col + 1] += current;
                }
            } else {
                new_counts[col] += current;
            }
        }

        counts = new_counts;
    }

    let total_timelines: i64 = counts.iter().sum();
    Ok(total_timelines)
}
