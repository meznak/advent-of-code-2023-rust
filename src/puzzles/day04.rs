use crate::RunError;

pub fn main(part: u8, data: String) -> Result<usize, RunError> {
    let parsed_data = parse_data(data)?;

    match part {
        1 => part1(&parsed_data),
        2 => part2(&parsed_data),
        _ => Err(RunError::BadPartNum),
    }
}

fn parse_data(data: String) -> Result<Vec<([usize; 5], [usize; 8])>, RunError> {
    let lines: Vec<&str> = data[..].split('\n').collect();
    let cards: Vec<([usize; 5], [usize; 8])> = vec![];

    for line in lines {
        let mut card: ([usize; 5], [usize; 8]);
        let sections: Vec<&str> = line.split(&[':', '|'][..]).collect();

        for card_half in 1 .. 3 {
            let mut numbers: Vec<&str> = sections[card_half].split(' ').collect();

            for (index, number) in numbers.enumerate() {
                card[card_half][index] = number.parse();
            }

            cards.push(card);
        }
    }
    Ok(cards)
}

fn part1(values: &Vec<([usize; 5], [usize; 8])>) -> Result<usize, RunError> {
    // Goal:

    todo!();
}

fn part2(values: &Vec<([usize; 5], [usize; 8])>) -> Result<usize, RunError> {
    // Goal:

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> String {
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .to_string()
    }

    fn sample_data() -> Vec<([usize; 5], [usize; 8])> {
        vec![
            ([41, 48, 83, 86, 17], [83, 86, 6, 31, 17, 9, 48, 53]),
            ([13, 32, 20, 16, 61], [61, 30, 68, 82, 17, 32, 24, 19]),
            ([1, 21, 53, 59, 44], [69, 82, 63, 72, 16, 21, 14, 1]),
            ([41, 92, 73, 84, 69], [59, 84, 76, 51, 58, 5, 54, 83]),
            ([87, 83, 26, 28, 32], [88, 30, 70, 12, 93, 22, 82, 36]),
            ([31, 18, 13, 56, 72], [74, 77, 10, 23, 35, 67, 36, 11]),
        ]
    }

    const SAMPLE_GOALS: [usize; 2] = [13, 0];

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
