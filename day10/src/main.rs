use aoc_utils::prelude::*;

mod board;
mod tiletype;

use board::Board;

fn main() -> PuzzleResult<()> {
    let _timer = Timer::new();
    let reader = get_puzzle_input()?;
    let board = Board::try_from(reader)?;

    let answer = if cfg!(feature = "part2") {
        board.count_enclosed_spaces()
    }
    else {
        board.find_furthest_distance()
    };

    println!("Answer is: {}", answer);

    Ok(())
}
