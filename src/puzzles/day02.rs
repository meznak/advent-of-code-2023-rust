use crate::RunError;

#[derive(Debug, PartialEq)]
struct Game {
    id: usize,
    cubes: Vec<[usize; 3]>
}

pub fn main(part: u8, data: &String) -> Result<usize, RunError> {
    let parsed_data = parse_data(data)?;

    match part {
        1 => part1(parsed_data),
        2 => part2(parsed_data),
        _ => Err(RunError::BadPartNum)
    }
}

fn parse_data(data: &String) -> Result<Vec<Game>, RunError> {
    let lines: Vec<&str> = data[..].split("\n:;").collect();

    todo!();
}

fn part1(values: Vec<Game>) -> Result<usize, RunError> {
    // Goal: Sum the IDs of possible games,
    // given 12 red, 13 green, 14 blue cubes.

    todo!();
}

fn part2(values: Vec<Game>) -> Result<usize, RunError> {
    // Goal:

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    const SAMPLE_DATA: &[Game] = &[
        Game{ id: 1, cubes: [[4, 0, 3], [1, 2, 6], [0, 2, 0]]},
        // Game{ id: 2, cubes: [[0, 2, 1], [1, 3, 4], [0, 1, 1]]},
        // Game{ id: 3, cubes: [[20, 8, 6], [4, 13, 5], [1, 5, 0]]},
        // Game{ id: 4, cubes: [[3, 1, 6], [6, 3, 0], [14, 3, 15]]},
        // Game{ id: 5, cubes: [[6, 3, 0], [1, 2, 2]]},
        ];

    const SAMPLE_DATA_2: &[Vec<Game>] = &[

        ];

    const SAMPLE_GOALS: [usize; 2] = [8, 0];

    #[test]
    fn test_parse() {
        assert_eq!(parse_data(&SAMPLE_INPUT.to_string()).unwrap(), SAMPLE_DATA);
    }

    #[test]
    fn test_part1() {
        let mut sample_data: Vec<Game> = vec![];
        SAMPLE_DATA.iter().for_each(|line| sample_data.push(*line));
        assert_eq!(part1(sample_data).unwrap(), SAMPLE_GOALS[0]);
    }

    #[test]
    fn test_part2() {
        let mut sample_data: Vec<Game> = vec![];
        SAMPLE_DATA.iter().for_each(|line| sample_data.push(*line));
        assert_eq!(part2(sample_data).unwrap(), SAMPLE_GOALS[1]);
    }
}