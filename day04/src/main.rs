fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("{}", output);
}

fn numbers_to_vec(numbers: &str) -> Vec<u32> {
    numbers
        .trim()
        .split(" ")
        .filter(|&n| !n.is_empty())
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn get_winning_numbers(card: &Card) -> Vec<&u32> {
    return card
        .your_numbers
        .iter()
        .filter(|&n| card.winning_numbers.contains(&n))
        .collect::<Vec<_>>();
}

fn get_card(line: &str) -> Card {
    dbg!(line);
    let parts = line.trim().split(":").collect::<Vec<&str>>();
    let id = parts[0].split(" ").last().unwrap().parse::<u32>().unwrap();
    let numbers = parts[1].split("|").collect::<Vec<&str>>();
    let winning_numbers = numbers_to_vec(numbers[0]);
    let your_numbers = numbers_to_vec(numbers[1]);

    return Card {
        id,
        winning_numbers,
        your_numbers,
    };
}

fn part1(lines: &str) -> u32 {
    return lines
        .lines()
        .map(|line| get_card(line))
        .filter(|card| get_winning_numbers(&card).len() > 0)
        .map(|card| u32::pow(2, get_winning_numbers(&card).len() as u32 - 1))
        .sum();
}

#[derive(Debug, PartialEq)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    your_numbers: Vec<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_card() {
        let lines: &str = "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let expected = [
            Card {
                id: 1,
                your_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
                winning_numbers: vec![41, 48, 83, 86, 17],
            },
            Card {
                id: 2,
                your_numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
                winning_numbers: vec![13, 32, 20, 16, 61],
            },
            Card {
                id: 3,
                your_numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
                winning_numbers: vec![1, 21, 53, 59, 44],
            },
            Card {
                id: 4,
                your_numbers: vec![59, 84, 76, 51, 58, 5, 54, 83],
                winning_numbers: vec![41, 92, 73, 84, 69],
            },
            Card {
                id: 5,
                your_numbers: vec![88, 30, 70, 12, 93, 22, 82, 36],
                winning_numbers: vec![87, 83, 26, 28, 32],
            },
            Card {
                id: 6,
                your_numbers: vec![74, 77, 10, 23, 35, 67, 36, 11],
                winning_numbers: vec![31, 18, 13, 56, 72],
            },
        ];

        for (i, line) in lines.trim().lines().enumerate() {
            let output = get_card(line);
            assert_eq!(output, expected[i]);
        }
    }

    #[test]
    fn test_numbers_to_vec() {
        let input = "1 2   3 4 5   ";
        let expected = vec![1, 2, 3, 4, 5];
        let output = numbers_to_vec(input);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_numbers_to_vec_with_empty() {
        let input = "1 2   3 4 5   ";
        let expected = vec![1, 2, 3, 4, 5];
        let output = numbers_to_vec(input);
        assert_eq!(output, expected);
    }
}
