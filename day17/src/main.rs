use std::io::Read;
use aoc_utils::prelude::*;

mod point;
mod worldmap;

use worldmap::WorldMap;

fn main() -> PuzzleResult<()> {
    let _timer = Timer::new();
    
    let mut buf = Vec::new();
    get_puzzle_input()?.read_to_end(&mut buf)?;

    let worldmap = WorldMap::try_from(buf)?;

    let best_route = worldmap.find_best_route(
        (0usize, 0usize).into(),
        (worldmap.width() - 1, worldmap.height() - 1).into()
    );

    println!("Answer: {}", best_route);

    Ok(())
}
