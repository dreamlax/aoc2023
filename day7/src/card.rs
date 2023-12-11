use std::fmt::{self,Debug,Display};
use aoc_utils::prelude::*;

#[derive(Eq,PartialEq,Ord,PartialOrd)]
pub struct Card(pub u32);

impl Card {
    pub fn from_char(c: char) -> Result<Self, PuzzleError> {
        match c {
            '2'..='9' => c
                .to_digit(10)
                .ok_or_else(|| PuzzleErrorKind::ParseError.into())
                .map(|c| Card(c)),
            'T' => Ok(Card(10)),
            'J' => if cfg!(feature="part2") {
                    Ok(Card(1))
                }
                else {
                    Ok(Card(11))
                }
            'Q' => Ok(Card(12)),
            'K' => Ok(Card(13)),
            'A' => Ok(Card(14)),
            _ => Err(PuzzleErrorKind::ParseError.into())
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self.0 {
            1 => 'j', // joker
            2..=9 => char::from_digit(self.0, 10)
                .ok_or_else(|| fmt::Error)?,
            10 => 'T',
            11 => 'J', // jack
            12 => 'Q',
            13 => 'K',
            14 => 'A',
            _ => return Err(fmt::Error)
        };

        write!(f, "{}", ch)
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
