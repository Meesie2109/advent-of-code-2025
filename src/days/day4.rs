pub fn solve_part1(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let height = lines.len();
    let width = lines[0].len();

    let mut result = 0;

    for y in 0..height {
        for x in 0..width {
            if lines[y][x] != '@' {
                continue;
            }

            let prev = if y > 0 { &lines[y - 1] } else { &Vec::new() };
            let curr = &lines[y];
            let next = if y + 1 < height {
                &lines[y + 1]
            } else {
                &Vec::new()
            };

            let neighbors = count_neighbors(x, [prev, curr, next]);

            if neighbors < 4 {
                result += 1;
            }
        }
    }

    Ok(result)
}

fn count_neighbors(idx: usize, rows: [&[char]; 3]) -> usize {
    let mut count = 0;

    for (row_i, row) in rows.iter().enumerate() {
        for dx in [-1, 0, 1] {
            let x = idx as isize + dx;

            if x < 0 || x as usize >= row.len() {
                continue;
            }
            if row_i == 1 && dx == 0 {
                continue;
            }

            if row[x as usize] == '@' {
                count += 1;
            }
        }
    }

    count
}
