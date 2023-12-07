use crate::RunError;

#[derive(Debug, PartialEq)]
struct PartNumber {
    start: [usize; 2],
    length: usize,
    value: usize,
    is_valid: bool
}

pub fn main(part: u8, data: &String) -> Result<usize, RunError> {
    let parsed_data = parse_data(data)?;

    match part {
        1 => part1(parsed_data),
        2 => part2(parsed_data),
        _ => Err(RunError::BadPartNum)
    }
}

fn parse_data(data: &String) -> Result<(Vec<PartNumber>, Vec<[usize; 2]>), RunError> {
    // let bytes = data.as_bytes();
    let mut part_numbers: Vec<PartNumber> = vec![];
    let mut symbols: Vec<[usize; 2]> = vec![];

    let (mut x, mut y) = (0, 0);
    let line_length = data.find('\n').unwrap();
    // let str_length = data.len();
    let mut cursor = -1;

    let mut characters = data.chars();

    loop {
        match characters.next() {
            None => { break; },
            Some(character) => {
                cursor += 1;
                if character.is_numeric() {
                    let mut num_str = character.to_string();
                    // let start = cursor;
                    let mut length = 1;
                    let mut value = 0;
                    let coords = [cursor % line_length, cursor / line_length];

                    // let mut cursor = index + 1;
                    // let mut finished = false;

                    loop {
                        let next_char = characters.next();
                        cursor += 1;

                        match next_char {
                            Some(c @ ('0' ..= '9')) => {
                                num_str.push_str(&c.to_string());
                                length += 1;
                            },
                            _ => {
                                value = num_str.parse().unwrap();
                                // finished = true;
                                break;
                            },
                        }
                    }

                    part_numbers.push(PartNumber { start: coords, length: length, value: value, is_valid: false });
                }
            }
        }
    }

    return Ok((part_numbers, symbols));
}

fn part1(values: (Vec<PartNumber>, Vec<[usize; 2]>)) -> Result<usize, RunError> {
    // Goal: Determine valid part numbers (adjacent to a symbol)
    // and return their sum.

    todo!();
}

fn part2(values: (Vec<PartNumber>, Vec<[usize; 2]>)) -> Result<usize, RunError> {
    // Goal:

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> String {
        "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..".to_string()
    }

    fn sample_data() -> (Vec<PartNumber>, Vec<[usize; 2]>) {
        (vec![PartNumber { start: [0,0], length: 3, value: 467, is_valid: true },
            PartNumber { start: [5, 0], length: 3, value: 114, is_valid: false },
            PartNumber { start: [2, 2], length: 2, value: 35, is_valid: true },
            PartNumber { start: [2, 6], length: 3, value: 633, is_valid: true },
            PartNumber { start: [4, 0], length: 3, value: 617, is_valid: true },
            PartNumber { start: [5, 8], length: 2, value: 58, is_valid: false },
            PartNumber { start: [6, 2], length: 3, value: 592, is_valid: true },
            PartNumber { start: [7, 6], length: 3, value: 755, is_valid: true },
            PartNumber { start: [9, 1], length: 3, value: 664, is_valid: true },
            PartNumber { start: [9, 5], length: 3, value: 598, is_valid: true }
        ],
        vec![
            [1, 3],
            [3, 6],
            [4, 3],
            [5, 5],
            [8, 3],
            [8, 5]
        ])
    }

    const SAMPLE_GOALS: [usize; 2] = [4361, 0];

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