use std::fmt::Debug;

fn parse1(line: &str) -> Game {
    let parts: Vec<_> = line.split(": ").collect();
    let id = parts[0]
        .trim()
        .split(" ")
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .expect("Cannot parse id");
    let mut cubes = Vec::new();

    for cube in parts[1].split(";") {
        let mut current_cubes = Cubes {
            red: 0,
            blue: 0,
            green: 0,
        };

        for color in cube.split(",") {
            let color_parts = color.trim().split(" ").collect::<Vec<_>>();
            let count = color_parts[0].parse::<u32>().expect("Bad count");
            let color_name = color_parts[1].trim();
            match color_name {
                "red" => current_cubes.red = count,
                "blue" => current_cubes.blue = count,
                "green" => current_cubes.green = count,
                _ => panic!("Unknown color"),
            }
        }

        cubes.push(current_cubes);
    }

    return Game {
        id: id,
        cubes: cubes,
    };
}

// return ids of games that are possible
fn process1(lines: Vec<&str>, cubes: &Cubes) -> Vec<u32> {
    let games = lines.iter().map(|&line| parse1(line)).collect::<Vec<_>>();
    return games
        .iter()
        .filter(|&game| {
            return game.cubes.iter().all(|cube| {
                return cube.red <= cubes.red
                    && cube.blue <= cubes.blue
                    && cube.green <= cubes.green;
            });
        })
        .map(|game| game.id)
        .collect();
}

fn part1(input: &str, cubes: &Cubes) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let ids = process1(lines, cubes);
    return ids.iter().sum();
}

struct Game {
    id: u32,
    cubes: Vec<Cubes>,
}

impl Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = format!("Game {{ id: {}, cubes: [", self.id);
        for cube in self.cubes.iter() {
            result.push_str(&format!(
                "Cubes {{ red: {}, blue: {}, green: {} }}, ",
                cube.red, cube.blue, cube.green
            ));
        }
        result.push_str("] }");
        write!(f, "{}", result)
    }
}

struct Cubes {
    red: u32,
    blue: u32,
    green: u32,
}

fn main() {
    let input = include_str!("input.txt");
    let output1 = part1(
        input,
        &Cubes {
            red: 12,
            blue: 14,
            green: 13,
        },
    );
    dbg!(output1);
}
// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse1() {
        let lines: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let games = [
            Game {
                id: 1,
                cubes: vec![
                    Cubes {
                        red: 4,
                        blue: 3,
                        green: 0,
                    },
                    Cubes {
                        red: 1,
                        blue: 6,
                        green: 2,
                    },
                    Cubes {
                        red: 0,
                        blue: 0,
                        green: 2,
                    },
                ],
            },
            Game {
                id: 2,
                cubes: vec![
                    Cubes {
                        red: 0,
                        blue: 1,
                        green: 2,
                    },
                    Cubes {
                        red: 1,
                        blue: 4,
                        green: 3,
                    },
                    Cubes {
                        red: 0,
                        blue: 1,
                        green: 1,
                    },
                ],
            },
            Game {
                id: 3,
                cubes: vec![
                    Cubes {
                        red: 20,
                        blue: 6,
                        green: 8,
                    },
                    Cubes {
                        red: 4,
                        blue: 5,
                        green: 13,
                    },
                    Cubes {
                        red: 1,
                        blue: 0,
                        green: 5,
                    },
                ],
            },
            Game {
                id: 4,
                cubes: vec![
                    Cubes {
                        red: 3,
                        blue: 6,
                        green: 1,
                    },
                    Cubes {
                        red: 6,
                        blue: 0,
                        green: 3,
                    },
                    Cubes {
                        red: 14,
                        blue: 15,
                        green: 3,
                    },
                ],
            },
            Game {
                id: 5,
                cubes: vec![
                    Cubes {
                        red: 6,
                        blue: 1,
                        green: 3,
                    },
                    Cubes {
                        red: 1,
                        blue: 2,
                        green: 2,
                    },
                ],
            },
        ];
        let output = lines.lines().map(|line| parse1(line)).collect::<Vec<_>>();
        for (i, game) in output.iter().enumerate() {
            dbg!(game, &games[i]);

            assert_eq!(game.id, games[i].id);
            assert_eq!(game.cubes.len(), games[i].cubes.len());

            for (j, cube) in game.cubes.iter().enumerate() {
                assert_eq!(cube.red, games[i].cubes[j].red);
                assert_eq!(cube.blue, games[i].cubes[j].blue);
                assert_eq!(cube.green, games[i].cubes[j].green);
            }
        }
    }

    #[test]
    fn test_process1() {
        let lines: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let output = process1(
            lines.lines().collect::<Vec<_>>(),
            &Cubes {
                red: 12,
                blue: 14,
                green: 13,
            },
        );
        assert_eq!(output, vec![1, 2, 5]);
    }

    #[test]
    fn test_part1() {
        let lines: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let output = part1(
            lines,
            &Cubes {
                red: 12,
                blue: 14,
                green: 13,
            },
        );
        assert_eq!(output, 8);
    }
}
