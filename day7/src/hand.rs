use std::fmt::{self,Debug,Display};
use aoc_utils::prelude::*;

use crate::card::Card;
#[derive(PartialOrd,Ord,Eq,PartialEq)]
pub struct Hand {
    pub hand_type: HandType,
    pub cards: [Card; 5]
}

#[derive(PartialOrd,Ord,PartialEq,Eq,Debug)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn determine_hand_type(cards: &[Card; 5]) -> HandType {
        let table: [u8; 15] = cards
            .iter()
            .fold([0u8; 15], |mut acc, c| { acc[c.0 as usize] += 1; acc });

        // count the number of unique cards, except jokers (jokers are at idx 1)
        let counts: Vec<&u8> = table[2..]
           .iter()
           .filter(|c| **c > 0)
           .collect();

        if table[1] == 0 {
            // no jokers
            match counts.len() {
                1 => HandType::FiveOfAKind,
                2 => if counts.iter().any(|c| **c == 4) {
                        HandType::FourOfAKind
                    }
                    else {
                        HandType::FullHouse
                    },
                3 => if counts.iter().any(|c| **c == 2) {
                        HandType::TwoPair
                    }
                    else {
                        HandType::ThreeOfAKind
                    },
                4 => HandType::OnePair,
                5 => HandType::HighCard,
                _ => panic!("Impossibru")
            }
        }
        else {
            // yes jokers
            match counts.len() {
                0 => HandType::FiveOfAKind, // all jokers
                1 => HandType::FiveOfAKind,
                2 => if counts.iter().all(|c| **c == 2) {
                        HandType::FullHouse
                    }
                    else {
                        HandType::FourOfAKind
                    }
                3 => HandType::ThreeOfAKind,
                4 => HandType::OnePair,
                _ => panic!("Impossibru")
            }
        }
    }

    pub fn from_str(s: &str) -> Result<Self,PuzzleError> {
        if s.len() != 5 {
            return Err(PuzzleErrorKind::ParseError.into());
        }

        let cards: Vec<Card> = s
            .chars()
            .map(|c| Card::from_char(c))
            .collect::<Result<Vec<_>,_>>()?;

        let cards: [Card; 5] = cards
            .try_into()
            .map_err(|_| PuzzleError::from(PuzzleErrorKind::ParseError))?;

        let hand_type = Self::determine_hand_type(&cards);
        
        Ok(Hand { cards, hand_type })
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}{}{}", self.cards[0], self.cards[1], self.cards[2], self.cards[3], self.cards[4])
    }
}

impl Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
