pub fn solve_part1(input: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = input.split("\n").collect();

    let mut result = 0;
    for part in parts {
        let mut number_1 = 0;
        let mut number_2 = 0;

        let mut index = 0;
        for (i, char) in part.chars().enumerate() {
            let num: i32 = char.to_digit(10).ok_or_else(|| "Char is not a number")? as i32;

            if num > number_1 {
                number_1 = num;
                index = i + 1;
            }
        }

        if index == part.chars().count() {
            let mut chars = part.chars().collect::<Vec<char>>();

            chars.pop();
            number_2 = number_1;
            number_1 = 0;

            for char in chars {
                let num: i32 = char.to_digit(10).ok_or_else(|| "Char is not a number")? as i32;

                if num > number_1 {
                    number_1 = num;
                }
            }
        }

        for char in part.chars().skip(index) {
            let num: i32 = char.to_digit(10).ok_or_else(|| "Char is not a number")? as i32;

            if num > number_2 {
                number_2 = num;
            }
        }

        result += format!("{}{}", number_1, number_2).parse::<i64>()?;
        println!("Result after {}", result);
    }

    Ok(result)
}
