use std::io::BufRead;
use aoc_utils::prelude::*;

mod card;
mod hand;

use hand::Hand;

fn main() -> PuzzleResult<()> {
    let _timer = Timer::new();
    let reader = get_puzzle_input()?;

    let mut hands_and_bids: Vec<(Hand,u32)> = Vec::new();
    for line in reader.lines() {
        let line = line?;

        let (hand, bid) = line.split_at(5);
        let hand = Hand::from_str(hand)?;
        let bid = u32::from_str_radix(&bid[1..], 10)?;

        hands_and_bids.push((hand, bid));
    }

    hands_and_bids.sort();

    #[cfg(debug_assertions)]
    for (h, _) in &hands_and_bids {
        println!("{} ({:?})", h, h.hand_type);
    }

    let total_winnings: u32 = hands_and_bids
        .iter()
        .enumerate()
        .map(|(idx, (_hand, bid))| (idx + 1) as u32 * bid)
        .sum();

    println!("Total winnings: {}", total_winnings);

    Ok(())
}
