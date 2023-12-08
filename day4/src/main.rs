use std::collections::{HashSet, HashMap};
use std::io::BufRead;
use aoc_utils::prelude::*;

struct Card {
    pub id: usize,
    pub winning_numbers: HashSet<u32>,
    pub game_numbers: HashSet<u32>
}

impl Card {
    pub fn get_winning_number_count(&self) -> usize {
        self.winning_numbers
            .intersection(&self.game_numbers)
            .count()
    }

    pub fn get_points(&self) -> u32 {
        match self.get_winning_number_count() {
            0 => 0,
            v => 1 << (v - 1)
        }
    }

    pub fn from_str(s: &str) -> PuzzleResult<Self> {
        if !s.starts_with("Card ") {
            return Err(PuzzleErrorKind::ParseError.into());
        }

        let colon = s
            .find(':')
            .ok_or_else(||
                PuzzleErrorKind::ParseError
            )?;

        let pipe = s
            .find('|')
            .ok_or_else(||
                PuzzleErrorKind::ParseError
            )?;

        let id = usize::from_str_radix(&s[5..colon].trim(), 10)
            .map_err(|_| PuzzleErrorKind::ParseError)?;

        let winning_numbers = s[colon+1..pipe]
            .split_ascii_whitespace()
            .map(|s|
                u32::from_str_radix(s, 10)
                    .map_err(|_| PuzzleErrorKind::ParseError.into())
            )
            .collect::<PuzzleResult<HashSet<u32>>>()?;

        let game_numbers = s[pipe+1..]
            .split_ascii_whitespace()
            .map(|s|
                u32::from_str_radix(s, 10)
                    .map_err(|_| PuzzleErrorKind::ParseError.into())
            )
            .collect::<PuzzleResult<HashSet<u32>>>()?;

        Ok(Self {
            id, winning_numbers, game_numbers
        })
    }
}

fn main() -> PuzzleResult<()> {
    let _timer = Timer::new();
    let reader = get_puzzle_input()?;

    let cards: Vec<Card> = reader
        .lines()
        .filter_map(|s| s.ok())
        .map(|s| Card::from_str(&s))
        .collect::<PuzzleResult<Vec<Card>>>()?;

    if cfg!(not(feature = "part2"))
    {
        let total_points: u32 = cards
            .iter()
            .map(|c| c.get_points())
            .sum();

        println!("Total points: {}", total_points);
    }
    else
    {
        let card_values: HashMap<usize, usize> = cards
            .iter()
            .map(|c| (c.id, c.get_winning_number_count()))
            .collect();

        let mut card_totals: HashMap<usize, usize> = HashMap::new();

        for card in cards {
            let this_total = card_totals.entry(card.id).or_insert(0);
            *this_total += 1;
            let this_total = *this_total;

            let next_cards_to_clone = card_values.get(&card.id).unwrap();
            for dup_id in card.id+1..=card.id+next_cards_to_clone {
                *card_totals.entry(dup_id).or_insert(0) += this_total;
            }
        }
        
        println!("Cards totals: {:?}", card_totals.values().sum::<usize>());
    }

    Ok(())
}
