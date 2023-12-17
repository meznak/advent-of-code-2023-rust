use crate::RunError;

pub fn main(part: u8, data: &String) -> Result<usize, RunError> {
    let parsed_data = parse_data(data)?;

    match part {
        1 => part1(&parsed_data),
        2 => part2(&parsed_data),
        _ => Err(RunError::BadPartNum),
    }
}

fn parse_data(data: &String) -> Result<Vec<(Vec<usize>, Vec<usize>)>, RunError> {
    let lines: Vec<&str> = data[..].split('\n').collect();
    let mut cards: Vec<(Vec<usize>, Vec<usize>)> = vec![];

    for line in lines {
        let mut card: (Vec<usize>, Vec<usize>) = (vec![], vec![]);
        let mut split: Vec<&str> = line.split(' ').collect();
        split.retain(|&x| x != "");
        split = split[2..].to_vec();

        let mut card_half = &mut card.0;
        for value in split {
            if value == "|" {
                card_half = &mut card.1;
                continue;
            }
            card_half.push(value.parse().unwrap());
        }

        cards.push(card);
        }
    Ok(cards)
}

fn part1(values: &Vec<(Vec<usize>, Vec<usize>)>) -> Result<usize, RunError> {
    // Goal: Submit sum of card scores.
    // First matching number is worth 1pt, each additional match doubles score

    let mut total_score = 0;
    let mut score: usize;

    for card in values {
        score = 0;
        for number in &card.1 {
            if card.0.contains(&number) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }

        total_score += score;
    }

    Ok(total_score)
}

fn part2(values: &Vec<(Vec<usize>, Vec<usize>)>) -> Result<usize, RunError> {
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

    fn sample_data() -> Vec<(Vec<usize>, Vec<usize>)> {
        vec![
            ([41, 48, 83, 86, 17].to_vec(), [83, 86,  6, 31, 17,  9, 48, 53].to_vec()),
            ([13, 32, 20, 16, 61].to_vec(), [61, 30, 68, 82, 17, 32, 24, 19].to_vec()),
            ([ 1, 21, 53, 59, 44].to_vec(), [69, 82, 63, 72, 16, 21, 14,  1].to_vec()),
            ([41, 92, 73, 84, 69].to_vec(), [59, 84, 76, 51, 58,  5, 54, 83].to_vec()),
            ([87, 83, 26, 28, 32].to_vec(), [88, 30, 70, 12, 93, 22, 82, 36].to_vec()),
            ([31, 18, 13, 56, 72].to_vec(), [74, 77, 10, 23, 35, 67, 36, 11].to_vec()),
        ]
    }

    const SAMPLE_GOALS: [usize; 2] = [13, 0];

    #[test]
    fn test_parse() {
        assert_eq!(parse_data(sample_input()).unwrap(), sample_data());
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&sample_data()).unwrap(), SAMPLE_GOALS[0]);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&sample_data()).unwrap(), SAMPLE_GOALS[1]);
    }
}
