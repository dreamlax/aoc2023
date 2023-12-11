use aoc_utils::prelude::*;

#[derive(Debug,Eq,PartialEq)]
pub enum TileType {
    Ground,
    NS,
    EW,
    NE,
    NW,
    SE,
    SW,
    Start
}

impl TileType {
    pub fn leads_south(&self) -> bool {
        *self == TileType::NS || *self == TileType::SE || *self == TileType::SW
    }

    pub fn leads_north(&self) -> bool {
        *self == TileType::NS || *self == TileType::NE || *self == TileType::NW
    }

    pub fn leads_east(&self) -> bool {
        *self == TileType::EW || *self == TileType::NE || *self == TileType::SE
    }

    pub fn leads_west(&self) -> bool {
        *self == TileType::EW || *self == TileType::NW || *self == TileType::SW
    }
}

impl TryFrom<char> for TileType {
    type Error = PuzzleError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(TileType::Ground),
            '|' => Ok(TileType::NS),
            '-' => Ok(TileType::EW),
            'L' => Ok(TileType::NE),
            'J' => Ok(TileType::NW),
            '7' => Ok(TileType::SW),
            'F' => Ok(TileType::SE),
            'S' => Ok(TileType::Start),
            _ => Err(PuzzleErrorKind::ParseError.into())
        }
    }
}
