use indicatif::ProgressIterator;
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, digit1, line_ending, space0, space1},
    combinator::map_res,
    multi::separated_list0,
    sequence::{separated_pair, tuple},
    IResult,
};
use rayon::prelude::*;

fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    println!("{}", output);
    let output2 = part2(input);
    println!("{}", output2);
}

#[derive(Debug)]
struct Range {
    src: u64,
    dst: u64,
    len: u64,
}

#[derive(Debug)]
struct Map<'a> {
    from: &'a str,
    to: &'a str,
    map: Vec<Range>,
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = tag("seeds:")(input)?;
    let (input, seed_array) =
        separated_list0(tag(" "), map_res(digit1, |s: &str| s.parse::<u64>()))(input.trim())?;

    return Ok((input, seed_array));
}

fn parse_seeds_ranges(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    let (input, _) = tag("seeds:")(input)?;
    let (input, seed_array) = separated_list0(
        tag(" "),
        separated_pair(
            map_res(digit1, |s: &str| s.parse::<u64>()),
            space1,
            map_res(digit1, |s: &str| s.parse::<u64>()),
        ),
    )(input.trim())?;

    return Ok((input, seed_array));
}

fn parse_mapping(input: &str) -> IResult<&str, (u64, u64, u64)> {
    let (input, (_, dst, _, src, _, len)) = tuple((
        space0,
        map_res(digit1, |s: &str| s.parse::<u64>()),
        space1,
        map_res(digit1, |s: &str| s.parse::<u64>()),
        space1,
        map_res(digit1, |s: &str| s.parse::<u64>()),
    ))(input)?;

    return Ok((input, (src, dst, len)));
}

// a map has the following format:
// <from>-to-<to> map:
// followed by a list of <to> <from> <length> lines
// which specify the start destination, start source and length of the keys
// any key which is not specified is assumed to be equal in both collections
fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, (_, (from, to))) =
        tuple((space0, separated_pair(alpha1, tag("-to-"), alpha1)))(input)?;
    let (input, _) = tuple((tag(" map:"), space0, line_ending))(input)?;
    let (input, mappings) = separated_list0(line_ending, parse_mapping)(input)?;

    let map = mappings
        .iter()
        .map(|(src, dst, len)| Range {
            src: *src,
            dst: *dst,
            len: *len,
        })
        .collect();

    return Ok((
        input,
        Map {
            from: from,
            to: to,
            map: map,
        },
    ));
}

/// Returns the mapped value for a given key from the provided map.
/// If the key is not found in the map, it returns the key itself.
///
/// # Arguments
///
/// * `map` - A reference to a Map struct which contains the mapping.
/// * `from` - The key for which to find the mapped value.
///
/// # Returns
///
/// * The mapped value if the key is found in the map, otherwise the key itself.
///
/// # Examples
///
/// ```
/// let mut map = Map::new();
/// map.map.insert(1, 2);
/// assert_eq!(get_mapping(&map, 1), 2);
/// assert_eq!(get_mapping(&map, 3), 3);
/// ```
fn get_mapping(map: &Map, from: u64) -> u64 {
    let found = map
        .map
        .iter()
        .find(|&r| r.src <= from && from < r.src + r.len);

    match found {
        Some(r) => r.dst + from - r.src,
        None => from,
    }
}

fn parse_maps(input: &str) -> IResult<&str, Vec<Map>> {
    let (input, maps) = separated_list0(tuple((line_ending, line_ending)), parse_map)(input)?;

    return Ok((input, maps));
}

fn find_dest(from: &str, value: u64, to: &str, maps: &Vec<Map>) -> Option<u64> {
    let mut i = 0;
    let mut cur_map = maps
        .iter()
        .find(|&m| m.from == from)
        .expect("Start map not found");
    let mut cur_value = get_mapping(cur_map, value);

    while i < maps.len() {
        cur_map = maps
            .iter()
            .find(|&m| m.from == cur_map.to)
            .expect("Map not found");
        cur_value = get_mapping(cur_map, cur_value);

        if cur_map.to == to {
            // println!(
            //     "Found {} {} for {} {} in {} steps",
            //     to, cur_value, from, value, i
            // );
            return Some(cur_value);
        }

        i += 1;
    }

    println!("No {} found for {} {}", to, from, value);
    return None;
}

fn get_seeds_and_maps(input: &str) -> IResult<&str, (Vec<u64>, Vec<Map>)> {
    let (input, seeds) = parse_seeds(input)?;
    let (input, _) =
        take_while1::<_, &str, nom::error::Error<_>>(|c| c == ' ' || c == '\n')(input)?;
    let (input, maps) = parse_maps(input)?;

    return Ok((input, (seeds, maps)));
}

fn get_seed_ranges_and_maps(input: &str) -> IResult<&str, (Vec<(u64, u64)>, Vec<Map>)> {
    let (input, seeds) = parse_seeds_ranges(input)?;
    let (input, _) =
        take_while1::<_, &str, nom::error::Error<_>>(|c| c == ' ' || c == '\n')(input)?;
    let (input, maps) = parse_maps(input)?;

    return Ok((input, (seeds, maps)));
}

fn part1(input: &str) -> u64 {
    let (_, (seeds, maps)) = get_seeds_and_maps(input).unwrap();

    return seeds
        .iter()
        .map(|s| find_dest("seed", *s, "location", &maps).unwrap())
        .min()
        .unwrap();
}

fn part2(input: &str) -> u64 {
    println!("Running part 2");
    let (_, (seeds_ranges, maps)) = get_seed_ranges_and_maps(input).unwrap();

    return seeds_ranges
        .par_iter()
        // .progress()
        .flat_map(|&(s, l)| {
            (s..s + l)
                .map(|val| find_dest("seed", val, "location", &maps))
                .min()
                .unwrap()
        })
        .min()
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    // declare shared input data
    const INPUT: &str = "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4";

    #[test]
    fn test_parse_seeds() {
        let seeds = "seeds: 79 14 55 13";
        let expected: Vec<u64> = vec![79, 14, 55, 13];
        match parse_seeds(seeds) {
            Ok((_, result)) => assert_eq!(result, expected),
            Err(_) => panic!("parse_seeds failed"),
        }
    }

    #[test]
    fn test_get_mapping() {
        let map = "seed-to-soil map:
        50 98 2
        52 50 48";
        // The entire list of seed numbers and their corresponding soil numbers looks like this:
        // seed  soil
        // 0     0
        // 1     1
        // ...   ...
        // 48    48
        // 49    49
        // 50    52
        // 51    53
        // ...   ...
        // 96    98
        // 97    99
        // 98    50
        // 99    51
        let seed_to_soil = vec![
            (0, 0),
            (1, 1),
            (48, 48),
            (49, 49),
            (50, 52),
            (51, 53),
            (96, 98),
            (97, 99),
            (98, 50),
            (99, 51),
        ];

        match dbg!(parse_map(map)) {
            Ok((_, result)) => {
                assert_eq!(result.from, "seed");
                assert_eq!(result.to, "soil");
                assert_eq!(result.map.len(), 2);
                for (seed, soil) in seed_to_soil {
                    assert_eq!(get_mapping(&result, seed), soil);
                }
            }
            Err(_) => panic!("parse_map failed"),
        }
    }

    #[test]
    fn test_find_dest() {
        let (_, (seeds, maps)) = get_seeds_and_maps(INPUT).unwrap();
        let val = find_dest("seed", seeds[0], "location", &maps).unwrap();
        assert_eq!(val, 82);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 35);
    }

    #[test]
    fn test_parse_seed_ranges() {
        let seeds = "seeds: 79 14 55 13";
        let expected: Vec<(u64, u64)> = vec![(79, 14), (55, 13)];
        match parse_seeds_ranges(seeds) {
            Ok((_, result)) => assert_eq!(result, expected),
            Err(_) => panic!("parse_seeds failed"),
        }
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 46);
    }
}
