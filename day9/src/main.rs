use std::io::BufRead;
use aoc_utils::prelude::*;

fn extrapolate(b: Vec<i32>) -> i32 {
    let mut diffs: Vec<Vec<i32>> = Vec::new();
    let mut current = Vec::from(b);

    loop {
        let x: Vec<i32> = current
            .windows(2)
            .map(|i| i[1] - i[0])
            .collect();
        diffs.insert(0, current);
        current = x;

        if current.iter().all(|x| *x == 0) {
            break
        }
    }

    if cfg!(not(feature = "part2")) {
        diffs
            .iter()
            .map(|v| v.last().unwrap())
            .sum()
    }
    else {
        diffs
            .iter()
            .fold(0, |acc,i| i.first().unwrap() - acc)
    }
}

fn main() -> PuzzleResult<()> {
    let _timer = Timer::new();
    let input = get_puzzle_input()?;

    let answer: i32 = input
        .lines()
        .map(|x| x.unwrap()
            .split_ascii_whitespace()
            .map(|s| i32::from_str_radix(s, 10).unwrap())
            .collect::<Vec<i32>>())
        .filter_map(|x| if x.len() > 0 { Some(extrapolate(x)) } else { None })
        .sum::<i32>();
    
    println!("Answer: {}", answer);

    Ok(())
}
