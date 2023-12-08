use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::io::BufRead;
use regex;
use aoc_utils::prelude::*;

#[derive(Clone)]
struct Coordinate {
    pub x: usize,
    pub y: usize
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let y_cmp = self.y.partial_cmp(&other.y);
        match y_cmp {
            Some(Ordering::Equal) => self.x.partial_cmp(&other.x),
            None => None,
            _ => y_cmp
        }
    }
}
impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        let y_cmp = self.y.cmp(&other.y);
        match y_cmp {
            Ordering::Equal => self.x.cmp(&other.x),
            _ => y_cmp
        }
    }
}
impl Eq for Coordinate {}

#[derive(Clone)]
struct Symbol {
    pub coordinate: Coordinate,
    pub symbol: char,
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.coordinate == other.coordinate
    }
}
impl PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.coordinate.partial_cmp(&other.coordinate)
    }
}
impl Ord for Symbol {
    fn cmp(&self, other: &Self) -> Ordering {
        self.coordinate.cmp(&other.coordinate)
    }
}
impl Eq for Symbol {}

struct Number {
    pub coordinate: Coordinate,
    pub value: u32,
    pub length: usize,
}
impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.coordinate == other.coordinate
    }
}
impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.coordinate.partial_cmp(&other.coordinate)
    }
}
impl Eq for Number {}

enum Token {
    Symbol(Symbol),
    Number(Number)
}

fn main() -> PuzzleResult<()> {
    let _timer = Timer::new();
    let reader = get_puzzle_input()?;

    let re = regex::Regex::new("(?<number>[0-9]+)|(?<symbol>[#%&*+/=@$-])|(?<dot>[.]+)")
        .expect("Regex is invalid");

    let mut tokens = Vec::new();

    for (line_number, line) in reader.lines().enumerate() {
        let line = line.expect("Cannot read line");
        for capture in re.captures_iter(&line) {
            if let Some(m) = capture.name("number") {
                tokens.push(Token::Number(Number {
                    coordinate: Coordinate {
                        x: m.start(),
                        y: line_number
                    },
                    value: u32::from_str_radix(m.as_str(), 10)
                        .map_err(|_| PuzzleErrorKind::ParseError)?,
                    length: m.len()
                }));
            }
            else if let Some(m) = capture.name("symbol") {
                tokens.push(Token::Symbol(Symbol {
                    coordinate: Coordinate { x: m.start(), y: line_number },
                    symbol: m.as_str().chars().nth(0).expect("Should be one char")
                }));
            }
        }
    }

    if cfg!(not(feature = "part2"))
    {
        let mut valid_number_coordinates: BTreeSet<Coordinate> = BTreeSet::new();

        let symbols = tokens
            .iter()
            .filter_map(|t| match t {
                Token::Symbol(v) => Some(v),
                _ => None
            });
        
        for symbol in symbols {
            let x_start = if symbol.coordinate.x > 0 { symbol.coordinate.x - 1 } else { 0 };
            let y_start = if symbol.coordinate.y > 0 { symbol.coordinate.y - 1 } else { 0 };
            let x_end = x_start + if symbol.coordinate.x > 0 { 3 } else { 2 };
            let y_end = y_start + if symbol.coordinate.y > 0 { 3 } else { 2 };
    
            for x in x_start..x_end {
                for y in y_start..y_end {
                    valid_number_coordinates.insert(Coordinate { x, y });
                }
            }
        }        

        let answer: u32 = tokens
            .iter()
            .filter_map(|t| match t {
                Token::Number(v) => Some(v),
                _ => None
            })
            .filter(|number| (0..number.length).any(|idx|
                valid_number_coordinates.contains(&Coordinate {
                    x: number.coordinate.x + idx,
                    y: number.coordinate.y
                })
            ))
            .map(|number| number.value)
            .sum();

        println!("Part 1 answer: {}", answer);
    }
    else
    {
        let mut valid_number_coordinates: BTreeMap<Coordinate, &Symbol> = BTreeMap::new();

        let symbols = tokens
            .iter()
            .filter_map(|t| match t {
                Token::Symbol(v) if v.symbol == '*' => Some(v),
                _ => None
            });

        for symbol in symbols {
            let x_start = if symbol.coordinate.x > 0 { symbol.coordinate.x - 1 } else { 0 };
            let y_start = if symbol.coordinate.y > 0 { symbol.coordinate.y - 1 } else { 0 };
            let x_end = x_start + if symbol.coordinate.x > 0 { 3 } else { 2 };
            let y_end = y_start + if symbol.coordinate.y > 0 { 3 } else { 2 };
    
            for x in x_start..x_end {
                for y in y_start..y_end {
                    valid_number_coordinates.insert(Coordinate { x, y }, &symbol);
                }
            }
        }

        let mut reverse_map: BTreeMap<&Symbol, Vec<&Number>> = BTreeMap::new();

        let numbers_next_to_gears = tokens
            .iter()
            .filter_map(|t| match t {
                Token::Number(v) => Some(v),
                _ => None
            })
            .filter_map(|number| {
                for idx in 0..number.length {
                    match valid_number_coordinates.get(&Coordinate { x: number.coordinate.x + idx, y: number.coordinate.y }) {
                        Some(symbol) => return Some((symbol, number)),
                        None => ()
                    }
                };
                None
            });
        
        for n in numbers_next_to_gears { 
            reverse_map.entry(*n.0).or_insert(Vec::new()).push(n.1);
        }

        let answer: u32 = reverse_map
            .values()
            .filter(|v| v.len() == 2)
            .map(|v| v.iter().map(|n| n.value).product::<u32>())
            .sum();

        println!("Part 2 answer: {}", answer);
    }

    Ok(())
}
