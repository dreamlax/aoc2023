use std::collections::BTreeMap;
use std::io::BufRead;
use aho_corasick::{AhoCorasick,Match};
use aoc_utils::prelude::*;

fn main() -> PuzzleResult<()> {
    let _timer = Timer::new();
    let reader = get_puzzle_input()?;

    let string_map: BTreeMap<&str, u32> =
        if cfg!(feature="part2") {
            BTreeMap::from([
                ("1", 1),
                ("2", 2),
                ("3", 3),
                ("4", 4),
                ("5", 5),
                ("6", 6),
                ("7", 7),
                ("8", 8),
                ("9", 9),
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9)
            ])
        }
        else {
            BTreeMap::from([
                ("1", 1),
                ("2", 2),
                ("3", 3),
                ("4", 4),
                ("5", 5),
                ("6", 6),
                ("7", 7),
                ("8", 8),
                ("9", 9)
            ])
        };

    let needles: Vec<&str> = string_map
        .keys()
        .copied()
        .collect();
    
    let aho = AhoCorasick::new(needles)
        .expect("Cannot create AhoCorasick");

    let mut total: u32 = 0;

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.len() == 0 {
            continue;
        }

        let all_matches: Vec<Match> = aho.find_overlapping_iter(&line).collect();
        if all_matches.len() == 0 {
            eprintln!("Invalid data: {}", line);
            return Err(PuzzleErrorKind::InputError.into());
        }

        let first = all_matches
            .first()
            .unwrap();

        let last = all_matches
            .last()
            .unwrap();

        let first_value = string_map
            .get(&line[first.start()..first.end()])
            .unwrap();

        let last_value = string_map
            .get(&line[last.start()..last.end()])
            .unwrap();


        let code = first_value * 10 + last_value;
        total += code;
    }

    println!("Total: {}", total);

    Ok(())
}
