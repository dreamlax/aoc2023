use std::collections::HashMap;
use std::io::BufRead;
use std::ops::Range;
use aoc_utils::prelude::*;

// turns out this wasn't needed...
mod rangeutils;

enum State {
    ParseSeeds,
    ParseTableName,
    ParseTableRow
}

type Mapping = Vec<(Range<i64>,i64)>;
type Almanac = HashMap<String,Mapping>;

fn main() -> PuzzleResult<()> {
    let _timer = Timer::new();
    let reader = get_puzzle_input()?;

    let mut almanac: Almanac = Almanac::new();
    let mut current_mapping_name: Option<String> = None;
    let mut current_mapping = Mapping::new();
    let mut seeds: Vec<Range<i64>> = Vec::new();
    
    let mut parse_state: State = State::ParseSeeds;

    // Chain an empty string to ensure the last table is parsed
    for line in reader.lines().chain([Ok(String::new())].into_iter()) {
        let line = line?;
        let line = line.trim();
        if line.len() == 0 {
            match current_mapping_name {
                Some(name) => {
                    almanac.insert(name, current_mapping);
                    current_mapping_name = None;
                    current_mapping = Mapping::new();
                    parse_state = State::ParseTableName;
                },
                None => { }
            }

            continue;
        }

        match parse_state {
            State::ParseSeeds => {
                if !line.starts_with("seeds: ") {
                    return Err(PuzzleErrorKind::ParseError.into());
                }

                if cfg!(not(feature = "part2")) {
                    seeds = line[7..]
                        .split_ascii_whitespace()
                        .map(|s| i64::from_str_radix(s, 10)
                            .map_err(|_| PuzzleErrorKind::ParseError.into()))
                        .collect::<PuzzleResult<Vec<i64>>>()?
                        .iter()
                        .map(|c| *c..*c+1)
                        .collect();
                }
                else {
                    seeds = line[7..]
                        .split_ascii_whitespace()
                        .map(|s| i64::from_str_radix(s, 10)
                            .map_err(|_| PuzzleErrorKind::ParseError.into()))
                        .collect::<PuzzleResult<Vec<i64>>>()?
                        .as_slice()
                        .chunks(2)
                        .map(|v| v[0]..v[0]+v[1])
                        .collect();
                }

                parse_state = State::ParseTableName;
            },

            State::ParseTableName => {
                if line.ends_with("map:") {
                    current_mapping_name = Some(line
                        .strip_suffix(" map:")
                        .ok_or_else(|| PuzzleErrorKind::ParseError)?
                        .trim()
                        .to_owned());
                }
                else {
                    return Err(PuzzleErrorKind::ParseError.into());
                }

                parse_state = State::ParseTableRow;
            },

            State::ParseTableRow => {
                let row_data: Vec<i64> = line
                    .split_ascii_whitespace()
                    .map(|s| i64::from_str_radix(s, 10)
                        .map_err(|_| PuzzleErrorKind::ParseError.into()))
                    .collect::<PuzzleResult<Vec<i64>>>()?;

                if row_data.len() != 3 {
                    return Err(PuzzleErrorKind::ParseError.into())
                }
                current_mapping.push((row_data[1]..(row_data[1] + row_data[2]), row_data[0]));
            }
        }
    }

    let transitions = [
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    // Go through each transition, updating the current seed ranges to their
    // destination ranges in the next map.
    for transition in transitions {
        let mapping = almanac
            .get_mut(transition)
            .ok_or_else(|| PuzzleErrorKind::InputError)?;

        // sorting means we can break early if we find a range intersection
        mapping.sort_unstable_by(|(left,_),(right,_)| left.start.cmp(&right.start));

        let mut index: usize = 0;
        loop {
            if index >= seeds.len() {
                break;
            }

            let current = seeds[index].clone();

            for (m, destination) in mapping.iter() {
                if m.contains(&current.start) && m.contains(&(current.end - 1)) {
                    // seed range is fully contained, so we can update this seed range
                    // with the narrowed destination range
                    seeds[index] = (*destination + current.start - m.start)..(*destination + current.end - m.start);
                    break;
                }
                else if !m.contains(&current.start) && m.contains(&(current.end - 1)) {
                    // seed range partially intersects the current mapped range (to the right)
                    seeds[index] = *destination..(*destination + current.end - m.start);
                    // keep remainder (for "fall through")
                    seeds.insert(index + 1, current.start..m.start);
                    break;
                }
                else if m.contains(&current.start) && !m.contains(&(current.end - 1)) {
                    // seed range partially intersects on the left
                    seeds[index] = (*destination + current.start - m.start)..(*destination + (m.end - m.start));
                    // keep remainder (for "fall through")
                    seeds.insert(index + 1, m.end..current.end);
                    break;
                }
            }
            
            index += 1;
        }
    }

    let answer = seeds.iter().map(|s| s.start).min().unwrap();

    println!("Answer: {}", answer);

    Ok(())
}
