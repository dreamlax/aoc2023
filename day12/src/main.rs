use std::io::BufRead;
use aoc_utils::prelude::*;

#[memoize::memoize]
fn recurse(springs: Vec<u8>, counts: Vec<u64>, current_run: u64) -> u64 {
    if counts.len() == 0 {
        // this recursion is good as long as there are no more '#'
        if current_run == 0 && springs.iter().all(|c| *c == b'.' || *c == b'?') {
            return 1;
        }
        else {
            return 0;
        }
    }

    if current_run > counts[0] {
        // this instantly ends a recursion
        return 0;
    }

    if springs.len() == 0 {
        // we have counts left, but no more input, so for this recursion to be
        // good, we need to make sure the current_run matches the one-and-only
        // count left
        if counts.len() == 1 && counts[0] == current_run {
            return 1;
        }
        else {
            return 0;
        }
    }

    match springs[0] {
        b'.' => {
            if current_run > 0 {
                if current_run == counts[0] {
                    // so far so good, keep going with the input/counts
                    return recurse(springs[1..].to_vec(), counts[1..].to_vec(), 0);
                }
                else {
                    // the run length is NG
                    return 0;
                }
            }
            else {
                // we're currently not in a run, just keep going
                return recurse(springs[1..].to_vec(), counts.clone(), 0);
            }
        },

        b'#' => {
            // if we find something that extends (or starts) the run, we keep
            // looking through
            return recurse(springs[1..].to_vec(), counts.clone(), current_run + 1);
        },

        b'?' => {
            if current_run > 0 {
                // Handle the case where "?" could continue or end a run
                if current_run == counts[0] {
                    return recurse(springs[1..].to_vec(), counts[1..].to_vec(), 0);
                }
                
                return recurse(springs[1..].to_vec(), counts.clone(), current_run + 1);
            }
            else {
                // Handle the case where "?" could start (or not start) a run
                return recurse(springs[1..].to_vec(), counts.clone(), 0)
                    + recurse(springs[1..].to_vec(), counts.clone(), 1);
            }
        }

        _ => panic!("Invalid input!")
    }
}

fn find_combinations(line: &str) -> PuzzleResult<u64> {
    let (springs, counts) = line
        .split_once(' ')
        .expect("Line should have a space in it");

    let counts = counts
        .split(',')
        .map(|s| u64::from_str_radix(s, 10))
        .collect::<Result<Vec<u64>,_>>()?;

    #[cfg(feature = "part2")]
    let counts = counts.repeat(5);

    #[cfg(feature = "part2")]
    let springs = Vec::from([springs])
        .repeat(5)
        .join("?");

    // shouldn't really do this, but given the input, we should be safe, and
    // it should be faster than applying ranges on strings
    let springs = springs.as_bytes();

    #[cfg(debug_assertions)]
    println!("Doing {springs:?} with {counts:?}");

    let result = recurse(springs.to_vec(), counts.to_vec(), 0);

    #[cfg(debug_assertions)]
    println!("Result: {}", result);

    Ok(result)
}

fn main() -> PuzzleResult<()> {
    let _timer = Timer::new();
    let reader = get_puzzle_input()?;

    let answer = reader
        .lines()
        .map(|line| find_combinations(&line.expect("Should be a line")))
        .sum::<Result<u64,_>>()?;

    println!("Answer: {}", answer);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let data = "???.### 1,1,3";

        let answer = find_combinations(&data).expect("This should work");

        assert_eq!(answer, 1, "Only one possibility");
    }

    #[test]
    fn test_case_2() {
        let data = ".??..??...?##. 1,1,3";

        let answer = find_combinations(&data).expect("This should work");

        assert_eq!(answer, 4, "4 possibilities");
    }

    #[test]
    fn test_case_3() {
        let data = "?###???????? 3,2,1";

        let answer = find_combinations(&data).expect("This should work");

        assert_eq!(answer, 10, "Should be 10");
    }
}
