use std::io::Read;
use aoc_utils::prelude::*;

/// Work out the slop between two strings (0 = equal, 1 = one char different, etc.)
fn sloppy_compare(line1: &str, line2: &str) -> usize {
    line1.chars()
        .zip(line2.chars())
        .fold(0, |acc,(c1,c2)| if c1 == c2 { acc } else { acc + 1 })
}

/// Check if the given mirror point is valid by comparing the the surrounding rows
/// and returning the amount of slop
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

/// Convert rows to columns
fn transpose(lines: &[&str]) -> Vec<String> {
    let transposed_len = lines[0].len();

    lines
        .iter()
        .fold(vec![String::with_capacity(lines.len()); transposed_len], |mut acc, line| {
            line
                .chars()
                .enumerate()
                .for_each(|(idx, c)| acc[idx].push(c));
            acc
        })
}

/// Look for the mirror point that has the required amount of slop. Returns None
/// if not found.
fn find_mirror_in_rows(rows: &[&str], slop: usize) -> Option<usize> {
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
        .nth(0)
}

/// Tries to find the mirror point in the given puzzle by first looking at
/// rows, and then looking at columns. The mirror point *must* contain the
/// provided level of slop.
fn find_mirror(s: &str, slop: usize) -> usize {
    let lines: Vec<&str> = s.split('\n').filter(|l| l.len() > 0).collect();

    if let Some(mirror_row) = find_mirror_in_rows(&lines, slop) {
        return mirror_row * 100;
    }

    let transposed = transpose(&lines);
    let transposed = transposed.iter().map(|x| x.as_str()).collect::<Vec<&str>>();
    find_mirror_in_rows(&transposed, slop)
        .expect("No mirror found in row or column")
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
