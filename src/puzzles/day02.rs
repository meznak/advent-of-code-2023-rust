use crate::RunError;
use regex::Regex;

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
    let lines: Vec<&str> = data[..].split("\n").collect();

    let re_game = Regex::new(r"(?P<game>\d+)$").unwrap();
    // let re_cubes = Regex::new(r"(?P<red>\d+) r|(?P<grn>\d+) g|(?P<blu>\d+) b").unwrap();
    let re_red = Regex::new(r"(\d+) r").unwrap();
    let re_grn = Regex::new(r"(\d+) g").unwrap();
    let re_blu = Regex::new(r"(\d+) b").unwrap();

    let mut games: Vec<Game> = vec![];

    for line in lines {
        let split: Vec<&str> = line.split(&[':', ';'][..]).collect();

        let Some(game_id) = re_game
            .captures(split[0])
        else { return Err(RunError::Regex(line.to_string())); };

        let mut game = Game { id: (&game_id[1]).parse::<usize>()?, cubes: Vec::new() };

        for group_num in 1..split.len() {
            match split.get(group_num) {
                Some(group) => {
                    let red: usize = re_red.captures(group).map_or("0", |m| m.get(1).unwrap().as_str()).parse().unwrap();
                    let grn: usize = re_grn.captures(group).map_or("0", |m| m.get(1).unwrap().as_str()).parse().unwrap();
                    let blu: usize = re_blu.captures(group).map_or("0", |m| m.get(1).unwrap().as_str()).parse().unwrap();
                    game.cubes.push([red, grn, blu]);
                },
                None => { return Err(RunError::Regex("line: {line}\ngroup_num: {group_num}".to_string())); }
            }
        };

        games.push(game);
    }

    Ok(games)
}

fn part1(values: Vec<Game>) -> Result<usize, RunError> {
    // Goal: Sum the IDs of possible games,
    // given 12 red, 13 green, 14 blue cubes.

    let mut id_sum = 0;
    let limits: [usize; 3] = [12, 13, 14];

    for game in values {
        let mut max: [usize; 3] = [0, 0, 0];
        let mut over_limit = false;
        for col in 0..3 {
            for group in &game.cubes {
                if group[col] > max[col] {
                    max[col] = group[col];
                }
            }
        }

        for col in 0..3 {
            if max[col] > limits[col] {
                over_limit = true;
                continue;
            }
        }

        if !over_limit {
            id_sum += game.id;
        }
    }

    Ok(id_sum)
}

fn part2(values: Vec<Game>) -> Result<usize, RunError> {
    // Goal:

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> String {
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()
    }

    fn sample_data() -> Vec<Game> {
        vec![
            Game{ id: 1, cubes: [[4, 0, 3], [1, 2, 6], [0, 2, 0]].to_vec()},
            Game{ id: 2, cubes: [[0, 2, 1], [1, 3, 4], [0, 1, 1]].to_vec()},
            Game{ id: 3, cubes: [[20, 8, 6], [4, 13, 5], [1, 5, 0]].to_vec()},
            Game{ id: 4, cubes: [[3, 1, 6], [6, 3, 0], [14, 3, 15]].to_vec()},
            Game{ id: 5, cubes: [[6, 3, 1], [1, 2, 2]].to_vec()},
        ]
    }

    const SAMPLE_GOALS: [usize; 2] = [8, 0];

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