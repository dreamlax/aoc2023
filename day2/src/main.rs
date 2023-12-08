use std::collections::BTreeMap;
use std::io::BufRead;
use aoc_utils::prelude::*;

#[derive(Eq,PartialEq,Ord,PartialOrd)]
enum Cube {
    Red,
    Blue,
    Green
}

struct Game {
    id: u32,
    rounds: Vec<BTreeMap<Cube, u32>>
}

fn parse_cube_count(s: &str) -> PuzzleResult<(Cube, u32)> {
    let space = s
        .find(' ')
        .ok_or_else(|| PuzzleErrorKind::ParseError)?;

    let color_name = &s[space+1..];

    let count = u32::from_str_radix(&s[..space], 10)
        .map_err(|_| PuzzleErrorKind::ParseError)?;

    let cube = match color_name {
        "red" => Cube::Red,
        "green" => Cube::Green,
        "blue" => Cube::Blue,
        _ => return Err(PuzzleErrorKind::ParseError.into())
    };

    Ok((cube, count))
}

fn parse_round(s: &str) -> PuzzleResult<BTreeMap<Cube, u32>> {
    let mut round = BTreeMap::new();
    for game in s.split(',') {
        let (cube, count) = parse_cube_count(game.trim())?;
        if let Some(sum) = round.get_mut(&cube) {
            *sum += count;
        }
        else {
            round.insert(cube, count);
        }
    }
    Ok(round)
}

impl Game {
    pub fn from_str(s: &str) -> PuzzleResult<Self> {
        if !s.starts_with("Game ") {
            return Err(PuzzleErrorKind::ParseError.into());
        }

        let colon = s
            .find(':')
            .ok_or_else(|| PuzzleErrorKind::ParseError)?;

        let id = u32::from_str_radix(&s[5..colon], 10)
            .map_err(|_| PuzzleErrorKind::ParseError)?;

        Ok(Self {
            id,
            rounds: s[colon+1..]
                .split(';')
                .map(parse_round)
                .collect::<PuzzleResult<Vec<BTreeMap<Cube,u32>>>>()?
        })
    }
}

fn main() -> PuzzleResult<()> {
    let _timer = Timer::new();
    let reader = get_puzzle_input()?;

    let games: Vec<Game> = reader
        .lines()
        .collect::<Result<Vec<String>,_>>()?
        .iter()
        .map(String::as_str)
        .map(Game::from_str)
        .collect::<PuzzleResult<Vec<Game>>>()?;

    if cfg!(not(feature = "part2"))
    {
        const RED_LIMIT: u32 = 12;
        const GREEN_LIMIT: u32 = 13;
        const BLUE_LIMIT: u32 = 14;

        let valid_games = games
            .iter()
            .filter(|g| {
                g.rounds.iter().all(|round|
                    *round.get(&Cube::Red).unwrap_or(&0) <= RED_LIMIT &&
                    *round.get(&Cube::Green).unwrap_or(&0) <= GREEN_LIMIT &&
                    *round.get(&Cube::Blue).unwrap_or(&0) <= BLUE_LIMIT
                )
            });

        let game_id_sum: u32 = valid_games
            .map(|g| g.id)
            .sum();

        println!("Game ID sum: {}", game_id_sum);
    }
    else
    {
        let answer: u32 = games
            .iter()
            .map(|g| 
                g.rounds.iter().map(|r| r.get(&Cube::Red).unwrap_or(&0)).max().unwrap_or(&0) *
                g.rounds.iter().map(|r| r.get(&Cube::Green).unwrap_or(&0)).max().unwrap_or(&0) *
                g.rounds.iter().map(|r| r.get(&Cube::Blue).unwrap_or(&0)).max().unwrap_or(&0)
            )
            .sum();

        println!("Sum of power of sets: {}", answer);
    }

    Ok(())
}
