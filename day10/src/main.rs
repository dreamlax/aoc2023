use aoc_utils::prelude::*;

mod board;
mod tiletype;

use board::Board;

fn main() -> PuzzleResult<()> {
    let _timer = Timer::new();
    let reader = get_puzzle_input()?;
    let board = Board::try_from(reader)?;

    let answer = if cfg!(not(feature="part2")) {
        board.find_furthest_distance()
    }
    else {
        board.count_enclosed_spaces()
    };

    println!("Answer is: {}", answer);

    Ok(())
}
