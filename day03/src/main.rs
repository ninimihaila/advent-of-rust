use core::num;
use std::fmt::Debug;

// return the position of the start and length of the numbers in a line
fn number_positions(line: &str) -> Option<Vec<(usize, usize)>> {
    let mut number_positions: Vec<(usize, usize)> = Vec::new();

    let line = line.trim();
    let mut i = 0;
    while i < line.len() {
        if line.chars().collect::<Vec<_>>()[i].is_digit(10) {
            let num = line[i..line.len()]
                .chars()
                .take_while(|c| c.is_digit(10))
                .count();
            number_positions.push((i, num));
            i += num;
        } else {
            i += 1;
        }
    }

    return if number_positions.len() > 0 {
        Some(number_positions)
    } else {
        None
    };
}

// return if the line has a symbol within the given position and one place to the left and right
fn has_symbol(line: &str, (start, len): (usize, usize)) -> bool {
    let line = line.trim();

    let window = if start == 0 {
        &line[start + len..start + len + 1]
    } else if start + len > line.len() {
        &line[start - 1..line.len()]
    } else if start + len == line.len() {
        &line[start - 1..start + len]
    } else {
        &line[start - 1..start + len + 1]
    };

    return window.contains(|c: char| !c.is_digit(10) && c != '.');
}

// return the "part numbers" of a line
// a part number is the number adjacent to a "symbol"
// a symbol is a character that is not a . or a number
fn part_numbers(lines: &str, line_number: usize) -> Vec<u32> {
    let lines = lines.lines().collect::<Vec<_>>();
    let mut part_numbers: Vec<u32> = Vec::new();

    let line = lines[line_number].trim();
    let line_above = if line_number > 0 {
        lines.get(line_number - 1)
    } else {
        None
    };
    let line_below = lines.get(line_number + 1);

    let line_numbers = number_positions(line).unwrap_or(vec![]);

    for (start, len) in line_numbers {
        match line_above {
            Some(&labv) => {
                if has_symbol(labv, (start, len)) {
                    part_numbers.push(line[start..start + len].parse::<u32>().unwrap());
                }
                continue;
            }
            None => {}
        }

        match line_below {
            Some(&lbel) => {
                if has_symbol(lbel, (start, len)) {
                    part_numbers.push(line[start..start + len].parse::<u32>().unwrap());
                }
                continue;
            }
            None => {}
        }

        if has_symbol(line, (start, len)) {
            part_numbers.push(line[start..start + len].parse::<u32>().unwrap());
        }
    }

    return part_numbers;
}

fn part1(lines: &str) -> u32 {
    return lines
        .lines()
        .enumerate()
        .flat_map(|(index, _line)| part_numbers(lines, index))
        // .flat_map(|(index, line)| part_numbers(lines.collect(), index))
        .sum();
}

fn main() {
    let input = include_str!("input.txt");
    let output1 = part1(input);
    dbg!(output1);
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_positions() {
        let lines: &str = "
          467..114..
          ...*......
          ..35..633.
          ......#...
          617*......
          .....+.58.
          ..592.....
          ......755.
          ...$.*....
          .664.598..";

        let expected = vec![
            Some(vec![(0, 3), (5, 3)]),
            None,
            Some(vec![(2, 2), (6, 3)]),
            None,
            Some(vec![(0, 3)]),
            Some(vec![(7, 2)]),
            Some(vec![(2, 3)]),
            Some(vec![(6, 3)]),
            None,
            Some(vec![(1, 3), (5, 3)]),
        ];
        for (i, line) in lines.trim().lines().enumerate() {
            let output = number_positions(line);
            assert_eq!(output, expected[i]);
        }
    }

    #[test]
    fn test_has_symbol() {
        let line_pos_expected = [
            ("467..114..", (0, 4), false),
            ("...*......", (2, 3), true),
            ("..35..633.", (9, 3), false),
            ("......#...", (4, 2), true),
            ("617*......", (2, 3), true),
            (".....+.58.", (6, 2), true),
            ("..592.....", (4, 4), false),
            ("......755.", (2, 2), false),
            ("...$.*....", (0, 3), true),
            (".664.598..", (6, 3), false),
        ];

        for (line, pos, expected) in line_pos_expected.iter() {
            let output = has_symbol(line, *pos);
            assert_eq!(output, *expected);
        }
    }

    #[test]
    fn test_part_numbers() {
        let lines: &str = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";

        let expected = [
            vec![467],
            vec![],
            vec![633],
            vec![],
            vec![617],
            vec![],
            vec![],
            vec![755],
            vec![],
            vec![664, 598],
        ];

        for (i, line) in lines.trim().lines().enumerate() {
            let output = part_numbers(lines, i);
            assert_eq!(output, expected[i]);
        }
    }

    #[test]
    fn test_part1() {
        let lines: &str = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";
        let output = part1(lines);
        assert_eq!(output, 4361);
    }
}
