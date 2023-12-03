use crate::RunError;

pub fn main(part: u8, data: &String) -> Result<usize, RunError> {
    let parsed_data = parse_data(data)?;

    match part {
        1 => part1(&parsed_data),
        2 => part2(&parsed_data),
        _ => Err(RunError::BadPartNum)
    }
}

fn parse_data(data: &String) -> Result<Vec<usize>, RunError> {
    let lines: Vec<&str> = data[..].split('\n').collect();

    match lines.iter().map(|x| Ok(x.trim().to_string())).collect() {
        Ok(parsed_data) => Ok(parsed_data),
        Err(e) => Err(RunError::ParseString(e)),
    }
}

fn part1(values: Vec<String>) -> Result<usize, RunError> {
    // Goal:

    todo!();
}

fn part2(values: Vec<String>) -> Result<usize, RunError> {
    // Goal:

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> String {
        "".to_string()
    }

    fn sample_data() -> String {
        "".to_string()
    }

    const SAMPLE_GOALS: [usize; 2] = [0, 0];

    #[test]
    fn test_parse() {
        assert_eq!(parse_data(&sample_input()).unwrap(), sample_data());
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(sample_data()).unwrap(), SAMPLE_GOALS[0]);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(sample_data()).unwrap(), SAMPLE_GOALS[1]);
    }
}