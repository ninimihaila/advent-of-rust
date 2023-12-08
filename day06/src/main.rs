use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, digit1, line_ending, space0, space1},
    combinator::map_res,
    multi::separated_list0,
    number,
    sequence::{separated_pair, tuple},
    IResult,
};

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn get_distance(button_hold: u128, race_time: u128) -> u128 {
    return (race_time - button_hold) * button_hold;
}

fn get_winning_margin(race_time: u128, record: u128) -> Vec<u128> {
    return (0..race_time)
        .map(|t| get_distance(t, race_time))
        .filter(|&d| d > record)
        .collect::<Vec<_>>();
}

fn multiply_all(v: Vec<u128>) -> u128 {
    return v.iter().fold(1, |acc, &x| acc as u128 * x);
}

fn get_labelled_array<'a>(input: &'a str, label: &'a str) -> IResult<&'a str, Vec<u128>> {
    let (input, _) = space0(input)?;
    let (input, _) = tag(label)(input)?;
    let (input, _) = space1(input)?;
    let (input, array) =
        separated_list0(space1, map_res(digit1, |s: &str| s.parse::<u128>()))(input)?;
    let (input, _) = take_while(|c: char| c.is_whitespace())(input)?;

    return Ok((input, array));
}

fn get_races(input: &str) -> Vec<(u128, u128)> {
    let (input, times) = get_labelled_array(input, "Time:").unwrap();
    let (_, distances) = get_labelled_array(input, "Distance:").unwrap();
    let records = times
        .iter()
        .zip(distances.iter())
        .map(|(&t, &d)| (t, d))
        .collect::<Vec<_>>();

    return records;
}

fn get_labelled_number<'a>(input: &'a str, label: &'a str) -> IResult<&'a str, u128> {
    let (input, _) = space0(input)?;
    let (input, _) = tag(label)(input)?;
    let (input, _) = space1(input)?;
    let (input, array) = separated_list0(space1, digit1)(input)?;
    let (input, _) = take_while(|c: char| c.is_whitespace())(input)?;

    // number is the numbers in the string array concatenated
    let number = array
        .iter()
        .fold(String::new(), |acc, &x| acc + x)
        .parse::<u128>()
        .unwrap();

    return Ok((input, number));
}

fn get_races_2(input: &str) -> (u128, u128) {
    let (input, times) = get_labelled_number(input, "Time:").unwrap();
    let (_, distances) = get_labelled_number(input, "Distance:").unwrap();

    return (times, distances);
}

fn part1(input: &str) -> u128 {
    let races = get_races(input);

    let margins = races
        .iter()
        .map(|(t, d)| get_winning_margin(*t, *d).len() as u128)
        .collect::<Vec<_>>();

    return multiply_all(margins);
}

fn part2(input: &str) -> u128 {
    let (race_time, record) = get_races_2(input);
    return get_winning_margin(race_time, record).len() as u128;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let measures = [
            (0, 0),
            (1, 6),
            (2, 10),
            (3, 12),
            (4, 12),
            (5, 10),
            (6, 6),
            (7, 0),
        ];

        let duration = 7;
        for (button_hold, expected) in measures.iter() {
            assert_eq!(get_distance(*button_hold, duration), *expected);
        }
    }

    #[test]
    fn test_winning_margin() {
        let races = [(7, 9), (15, 40), (30, 200)];
        let ways_to_win = [4, 8, 9];
        let expected = races.iter().zip(ways_to_win.iter());

        for ((duration, record), expected) in expected {
            let distances = get_winning_margin(*duration, *record);
            assert_eq!(distances.len(), *expected);
        }
    }

    #[test]
    fn test_get_races() {
        let input = "Time: 7 15 30
        Distance: 9 40 200";

        let expected = [(7, 9), (15, 40), (30, 200)];

        let parsed = get_races(input);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_part1() {
        let input = "Time: 7 15 30
           Distance: 9 40 200";
        assert_eq!(part1(input), 288);
    }

    #[test]
    fn test_part2() {
        let input = "Time: 7 15 30
      Distance: 9 40 200";
        assert_eq!(part2(input), 71503);
    }
}
