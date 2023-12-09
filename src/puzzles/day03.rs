use crate::RunError;

#[derive(Debug, PartialEq)]
struct PartNumber {
    start: [usize; 2],
    length: usize,
    value: usize,
    is_valid: bool
}

#[derive(Debug, PartialEq)]
struct Symbol {
    position: [usize; 2],
    character: char
}

pub fn main(part: u8, data: &String) -> Result<usize, RunError> {
    let mut parsed_data = parse_data(data)?;

    match part {
        1 => part1(&mut parsed_data),
        2 => part2(&mut parsed_data),
        _ => Err(RunError::BadPartNum)
    }
}

fn parse_data(data: &String) -> Result<(Vec<PartNumber>, Vec<Symbol>), RunError> {
    let mut part_numbers: Vec<PartNumber> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    let line_length = data.find('\n').unwrap();
    let mut cursor = 0;
    let mut char_buffer: Option<char> = None;

    let mut characters = data.chars();
    let mut character_opt: Option<char>;

    loop {
        character_opt = char_buffer.or_else(|| {characters.next()});
        char_buffer = None;

        match character_opt {
            None => { break; },
            Some(character @ ('.' | '\n' | 'a' | 'b')) => {
                if character == '\n' { cursor -= 1; };
            },
            Some(character) => {
                if character.is_numeric() {
                    let mut num_str = character.to_string();
                    let mut length = 1;
                    let value;
                    let coords = [cursor as usize / line_length,
                                              cursor as usize  % line_length];

                    loop {
                        let next_char = characters.next();

                        match next_char {
                            Some(c @ ('0' ..= '9')) => {
                                num_str.push_str(&c.to_string());
                                cursor += 1;
                                length += 1;
                            },
                            _ => {
                                value = num_str.parse().unwrap();
                                char_buffer = next_char;
                                break;
                            },
                        }
                    }

                    part_numbers.push(PartNumber { start: coords, length: length, value: value, is_valid: false });
                }
                else {
                    let coords = [cursor / line_length,
                                              cursor % line_length];
                    symbols.push(Symbol { position: coords, character: character });
                }
            }
        }
        cursor += 1;
    }

    return Ok((part_numbers, symbols));
}

fn part1(values: &mut (Vec<PartNumber>, Vec<Symbol>)) -> Result<usize, RunError> {
    // Goal: Determine valid part numbers (adjacent to a symbol)
    // and return their sum.

    for number in &mut values.0 {
        number.is_valid = check_is_valid(number, &values.1);
    }

    let mut sum = 0;

    for number in &values.0 {
        if number.is_valid {
            sum += number.value;
        }
    }

    Ok(sum)
}

fn part2(values: &mut (Vec<PartNumber>, Vec<Symbol>)) -> Result<usize, RunError> {
    // Goal:

    todo!();
}

fn check_is_valid(number: &PartNumber, symbols: &Vec<Symbol>) -> bool {
    let mut cursor = number.start;
    let mut height = 2;
    let mut length = number.length + 1;

    // before
    if cursor[1] > 0 {
        cursor[1] -= 1;
        length += 1;
    }

    // above
    if cursor[0] > 0 {
        cursor[0] -= 1;
        height += 1;
    }
     for y in cursor[0] .. cursor[0] + height {
        for x in cursor[1] .. cursor[1] + length {
            for symbol in symbols {
                if [y, x] == symbol.position {
                    return true;
                }
            }
        }
    }
    false
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

    fn sample_data() -> (Vec<PartNumber>, Vec<Symbol>) {
        (vec![PartNumber { start: [0,0], length: 3, value: 467, is_valid: false },
            PartNumber { start: [0, 5], length: 3, value: 114, is_valid: false },
            PartNumber { start: [2, 2], length: 2, value: 35, is_valid: false },
            PartNumber { start: [2, 6], length: 3, value: 633, is_valid: false },
            PartNumber { start: [4, 0], length: 3, value: 617, is_valid: false },
            PartNumber { start: [5, 7], length: 2, value: 58, is_valid: false },
            PartNumber { start: [6, 2], length: 3, value: 592, is_valid: false },
            PartNumber { start: [7, 6], length: 3, value: 755, is_valid: false },
            PartNumber { start: [9, 1], length: 3, value: 664, is_valid: false },
            PartNumber { start: [9, 5], length: 3, value: 598, is_valid: false }
        ],
        vec![
            Symbol { position: [1, 3], character: '*' },
            Symbol { position: [3, 6], character: '#' },
            Symbol { position: [4, 3], character: '*' },
            Symbol { position: [5, 5], character: '+' },
            Symbol { position: [8, 3], character: '$' },
            Symbol { position: [8, 5], character: '*' }
        ])
    }

    const SAMPLE_GOALS: [usize; 2] = [4361, 0];

    #[test]
    fn test_parse() {
        assert_eq!(parse_data(&sample_input()).unwrap(), sample_data());
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&mut sample_data()).unwrap(), SAMPLE_GOALS[0]);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&mut sample_data()).unwrap(), SAMPLE_GOALS[1]);
    }
}