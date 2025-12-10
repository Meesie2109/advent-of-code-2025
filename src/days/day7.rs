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
