use crate::RunError;

pub fn main(part: u8, data: &str) -> Result<usize, RunError> {
    let parsed_data = parse_data(data)?;

    match part {
        1 => part1(&parsed_data),
        2 => part2(&parsed_data),
        _ => Err(RunError::BadPartNum),
    }
}

fn parse_data(data: &str) -> Result<Vec<&str>, RunError> {
    let lines: Vec<&str> = data[..].split('\n').collect();

    match lines.iter().map(|x| Ok(x.trim())).collect() {
        Ok(parsed_data) => Ok(parsed_data),
        Err(e) => Err(RunError::ParseString(e)),
    }
}

fn part1(values: &[&str]) -> Result<usize, RunError> {
    // Goal: Concatenate first and last digit for each line. Sum all lines.

    let mut total = 0;

    values.iter().for_each(|line| {
        for character in (*line).chars() {
            if let Some(digit) = maybe_convert_char_to_digit(character) {
                total += digit * 10;
                break;
            }
        }

        for character in (*line).chars().rev().collect::<Vec<_>>() {
            if let Some(digit) = maybe_convert_char_to_digit(character) {
                total += digit;
                break;
            }
        }
    });

    Ok(total.try_into().unwrap())
}

fn part2(values: &[&str]) -> Result<usize, RunError> {
    // What's the goal?

    todo!();
}

fn maybe_convert_char_to_digit(character: char) -> Option<u32> {
    character.to_digit(10)
}

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE_INPUT: &str = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";
    static SAMPLE_DATA: &'static [&str] = &["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
    static SAMPLE_GOALS: [usize; 2] = [142, 0];

    #[test]
    fn test_parse() {
        assert_eq!(parse_data(&SAMPLE_INPUT).unwrap(), SAMPLE_DATA);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&SAMPLE_DATA).unwrap(), SAMPLE_GOALS[0]);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&SAMPLE_DATA).unwrap(), SAMPLE_GOALS[1]);
    }
}
