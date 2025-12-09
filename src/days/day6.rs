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

pub fn solve_part2(input: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let reversed_lines: Vec<Vec<char>> = lines.into_iter().rev().collect();

    let height = reversed_lines.len();
    let width = reversed_lines[0].len();

    let mut final_numbers: Vec<Vec<String>> = Vec::new();
    let mut iteration = 0;
    let mut action: char = '+';
    for x in 0..width {
        let mut temp_numbers: Vec<String> = Vec::new();
        for y in 0..height {
            let char = &reversed_lines[y][x];

            if y == 0 {
                if char == &'*' || char == &'+' {
                    action = *char;
                    continue;
                } else {
                    continue;
                }
            }

            if temp_numbers.len() <= y {
                temp_numbers.push(char.to_string());
            } else {
                temp_numbers[y].insert(0, *char);
            }
        }

        if temp_numbers.iter().all(|s| s.trim().is_empty()) {
            final_numbers[iteration].insert(0, action.to_string());

            iteration += 1;
        } else {
            let mut num = "".to_string();
            for number in &temp_numbers {
                num = format!("{}{}", number, num);
            }

            if final_numbers.len() <= iteration {
                final_numbers.push(vec![num.to_string()]);
            } else {
                final_numbers[iteration].push(num);
            }
        }
    }
    final_numbers[iteration].insert(0, action.to_string());

    let mut result = 0;
    for numbers in final_numbers {
        let mut action = '+';
        let mut count = 0;
        for num in numbers {
            let num_str = num.trim();

            if num_str == "*" || num_str == "+" {
                action = num.parse::<char>()?;
                continue;
            }

            let number = num_str.parse::<i64>()?;

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

        result += count;
    }

    Ok(result)
}

// 409176996966;
// 10951882745757
