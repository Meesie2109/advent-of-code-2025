pub fn solve_part1(input: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let lines: Vec<Vec<String>> = input
        .lines()
        .map(|l| {
            let mut items: Vec<String> = Vec::new();
            let mut item = String::new();

            for ch in l.chars() {
                if ch != ' ' {
                    item.push(ch);
                } else {
                    if !item.is_empty() {
                        items.push(item.clone());
                        item.clear();
                    }
                }
            }

            if !item.is_empty() {
                items.push(item);
            }

            items
        })
        .collect();

    let reversed_lines: Vec<Vec<String>> = lines.into_iter().rev().collect();

    let height = reversed_lines.len();
    let width = reversed_lines[0].len();

    let mut result = 0;
    for x in 0..width {
        let mut count = 0;
        let mut action: char = '+';
        for y in 0..height {
            let item = &reversed_lines[y][x];

            if item == "*" || item == "+" {
                action = item.parse::<char>()?;
            } else {
                let number = item.parse::<i64>()?;

                match action {
                    '*' => {
                        if count == 0 {
                            count += number;
                            continue;
                        }
                        count = count * number
                    }
                    '+' => count += number,
                    _ => {
                        println!("Action not found")
                    }
                }
            }
        }

        result += count;
    }

    Ok(result)
}
