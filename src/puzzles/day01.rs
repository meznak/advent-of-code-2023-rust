use crate::RunError;

pub fn main(part: u8, data: &String) -> Result<usize, RunError> {
    let parsed_data = parse_data(data)?;

    match part {
        1 => part1(parsed_data),
        2 => part2(parsed_data),
        _ => Err(RunError::BadPartNum),
    }
}

fn parse_data(data: &String) -> Result<Vec<String>, RunError> {
    let lines: Vec<&str> = data[..].split('\n').collect();

    match lines.iter().map(|x| Ok(x.trim().to_string())).collect() {
        Ok(parsed_data) => Ok(parsed_data),
        Err(e) => Err(RunError::ParseString(e)),
    }
}

fn part1(values: Vec<String>) -> Result<usize, RunError> {
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

fn part2(values: Vec<String>) -> Result<usize, RunError> {
    // Same as part 1, but spelled-out digits count, too.

    let mut parsed_values: Vec<String> = vec![];
    for line in values {
        let new_line = line.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "4")
        .replace("five", "5e")
        .replace("six", "6")
        .replace("seven", "7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");

        parsed_values.push(new_line);
    }

    part1(parsed_values)
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

    static SAMPLE_DATA: &[&str] = &[
        "1abc2",
        "pqr3stu8vwx",
        "a1b2c3d4e5f",
        "treb7uchet"];

    static SAMPLE_DATA_2: &[&str] = &[
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen"];

    static SAMPLE_GOALS: [usize; 2] = [142, 281];

    #[test]
    fn test_parse() {
        assert_eq!(parse_data(&SAMPLE_INPUT.to_string()).unwrap(), SAMPLE_DATA);
    }

    #[test]
    fn test_part1() {
        let mut sample_data: Vec<String> = vec![];
        SAMPLE_DATA.iter().for_each(|line| sample_data.push((*line).to_string()));
        assert_eq!(part1(sample_data).unwrap(), SAMPLE_GOALS[0]);
    }

    #[test]
    fn test_part2() {
        let mut sample_data: Vec<String> = vec![];
        SAMPLE_DATA_2.iter().for_each(|line| sample_data.push((*line).to_string()));
        assert_eq!(part2(sample_data).unwrap(), SAMPLE_GOALS[1]);
    }
}
