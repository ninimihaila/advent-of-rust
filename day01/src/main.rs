use std::collections::HashMap;

fn main() {
    let input= include_str!("input.txt");
    let output1 = part1(input);
    dbg!(output1);
    let output2 = part2(input);
    dbg!(output2);
}


fn get_first_and_last_digits(row: &str) -> (u32, u32) {
  let digits: Vec<_> = row
    .chars()
    .filter_map(|c| c.to_digit(10))
    .collect();

  (digits.first().unwrap().clone(), digits.last().unwrap().clone())
}

fn part1(input: &str) -> u32 {
  return input
    .lines()
    // .filter(|row| { row.is_empty() })  // why is this row double &&?
    .map(|row| {
      if row.is_empty() {
        return 0
      }

      let (first, last) = get_first_and_last_digits(row);
      first * 10 + last
    })
    .sum()
}

// const DIGIT_WORDS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
// const DIGITS: [&str; 9] = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
fn digit_map() -> HashMap<&'static str, u32> {
  return [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
  ].iter().cloned().collect();
}

fn get_first_and_last_digits_improved(row: &str) -> (u32, u32) {
  let map = digit_map();

  let mut it = (0..row.len()).filter_map(|index| {
    let rest_line = &row[index..];

    let next = map.keys().find(|&&key| rest_line.starts_with(key));

    return match next {
      Some(&num) => Some(map[num]),
      None => None,
    };
  });

  let first = it.next().expect("should have a number");
  let last = match it.last() {
    Some(num) => num,
    None => first,
  };

  return (first, last)
  // return (digits.first().unwrap().clone(), digits.last().unwrap().clone())
}

fn part2(input: &str) -> u32 {
  return input
    .lines()
    // .filter(|row| { row.is_empty() })  // why is this row double &&?
    .map(|row| {
      if row.is_empty() {
        return 0
      }

      let (first, last) = get_first_and_last_digits_improved(row);
      first * 10 + last
    })
    .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines: &str = "
          1abc2
          pqr3stu8vwx
          a1b2c3d4e5f
          treb7uchet";
        let output = part1(lines);
        assert_eq!(output, 142);
    }

    #[test]
    fn test_get_first_and_last_digits_improved() {
      let lines = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
      let expected= [
        (2, 9),
        (8, 3),
        (1, 3),
        (2, 4),
        (4, 2),
        (1, 4),
        (7, 6),
      ];

      for (i, line) in lines.lines().enumerate() {
        assert_eq!(expected[i], get_first_and_last_digits_improved(line))
      }
    }

    #[test]
    fn test_part2() {
        let lines = "two1nine
          eightwothree
          abcone2threexyz
          xtwone3four
          4nineeightseven2
          zoneight234
          7pqrstsixteen";
        let output: u32 = part2(lines);
        assert_eq!(output, 281);
    }
}
