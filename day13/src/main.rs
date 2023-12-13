use std::io::Read;
use aoc_utils::prelude::*;

fn sloppy_compare(line1: &str, line2: &str) -> usize {
    line1.chars()
        .zip(line2.chars())
        .fold(0, |acc,(c1,c2)| if c1 == c2 { acc } else { acc + 1 })
}

fn check_mirror_rows(lines: &[&str], mirror_point: usize) -> usize {
    assert!(lines.len() > 1);

    if mirror_point == 0 || mirror_point == lines.len() - 2 {
        return 0;
    }

    let rows_to_compare = (lines.len() - mirror_point - 2).min(mirror_point);
    let mut slop = 0;
    for idx in 1..=rows_to_compare {
        slop += sloppy_compare(lines[mirror_point - idx], lines[mirror_point + idx + 1]);
    }

    slop
}

fn transpose(lines: &[&str]) -> Vec<String> {
    let transposed_len = lines[0].len();

    lines
        .iter()
        .fold(vec![String::new(); transposed_len], |mut acc, line| {
            line
                .chars()
                .enumerate()
                .for_each(|(idx, c)| acc[idx].push(c));
            acc
        })
}

fn find_mirror_in_rows(rows: &[&str], slop: usize) -> Vec<usize> {
    rows
        .windows(2)
        .enumerate()
        .filter_map(|(idx, rows)| {
            let s = sloppy_compare(rows[0], rows[1]);
            if s <= slop {
                Some((s, idx))
            }
            else {
                None
            }
        })
        .filter_map(|(existing_slop, idx)| {
            let slop_remaining = slop - existing_slop;
            if check_mirror_rows(&rows, idx) == slop_remaining {
                Some(idx + 1)
            }
            else {
                None
            }
        })
        .collect::<Vec<usize>>()
}

fn find_mirror(s: &str, slop: usize) -> usize {
    let lines: Vec<&str> = s.split('\n').filter(|l| l.len() > 0).collect();

    let mirror_rows = find_mirror_in_rows(&lines, slop);
    if mirror_rows.len() == 0 {
        let transposed = transpose(&lines);
        let transposed = transposed.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
        let mirror_cols = find_mirror_in_rows(&transposed, slop);
        *mirror_cols.first().unwrap()
    }
    else {
        *mirror_rows.first().unwrap() * 100
    }
}

fn main() -> PuzzleResult<()> {
    let _timer = Timer::new();
    let mut buf = String::new();
    get_puzzle_input()?.read_to_string(&mut buf)?;

    let slop = if cfg!(feature = "part2") {
        1
    }
    else {
        0
    };

    let answer: usize = buf
        .split("\n\n")
        .into_iter()
        .map(|m| find_mirror(m, slop))
        .sum();

    println!("Answer: {answer}");

    Ok(())
}
