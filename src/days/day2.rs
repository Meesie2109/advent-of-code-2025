pub fn solve_part1(input: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let records: Vec<&str> = input.split_terminator(',').collect();

    let mut final_number = 0;
    for record in records {
        if let Some((part1, part2)) = record.split_once("-") {
            let number1: i64 = part1.parse()?;
            let number2: i64 = part2.parse()?;

            for num in number1..=number2 {
                let chars: Vec<char> = num.to_string().chars().collect();

                if (chars.len() % 2) == 1 {
                    continue;
                }

                let middle = chars.len() / 2;
                for i in 0..middle {
                    if chars[i] != chars[i + middle] {
                        break;
                    }

                    if (i + middle) == (chars.len()) - 1 {
                        final_number += num;
                    }
                }
            }
        }
    }

    Ok(final_number)
}
