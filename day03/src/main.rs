use std::fmt::Debug;

// return the "part numbers" of a line
// a part number is the number adjacent to a "symbol"
// a symbol is a character that is not a . or a number
fn part_numbers(lines: Vec<&str>, line_number: usize) -> Vec<u32> {
    let mut part_numbers: Vec<u32> = Vec::new();
    let line = lines[line_number].trim();
    let line_above = lines[line_number - 1].trim();
    let line_below = lines[line_number + 1].trim();

    return part_numbers;
}

fn part1(lines: &str) -> u32 {
    return lines
        .lines()
        .enumerate()
        .map(|(index, line)| part_numbers(line, index))
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
        assert_eq!(output, 8);
    }
}
