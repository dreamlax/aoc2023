use std::env;
use std::fs::File;
use std::io::BufReader;
use crate::error::PuzzleErrorKind;
use crate::result::PuzzleResult;

pub fn get_puzzle_input() -> PuzzleResult<BufReader<File>> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| PuzzleErrorKind::MissingInput)?;

    let input = File::open(filename)
        .map_err(|_| PuzzleErrorKind::InputError)?;

    Ok(BufReader::new(input))
}
