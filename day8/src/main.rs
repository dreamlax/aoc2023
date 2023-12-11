use std::collections::HashMap;
use std::io::BufRead;
use aoc_utils::prelude::*;

#[derive(Eq,PartialEq)]
enum Instruction {
    Left,
    Right
}

#[derive(Eq,PartialEq)]
struct Destination {
    left: String,
    right: String
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        if b < a {
            std::mem::swap(&mut a, &mut b);
        }
        b %= a;
    }
    a
}

pub fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

impl TryFrom<char> for Instruction {
    type Error = PuzzleError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            _ => Err(PuzzleErrorKind::ParseError.into())
        }
    }
}

fn parse_map_line<S: AsRef<str>>(line: S) -> PuzzleResult<(String, Destination)> {
    let line = line.as_ref();
    let equals = line
        .find('=')
        .ok_or_else(|| PuzzleError::from(PuzzleErrorKind::ParseError))?;

    let left_paren = equals + line[equals..]
        .find('(')
        .ok_or_else(|| PuzzleError::from(PuzzleErrorKind::ParseError))?;

    let comma = left_paren + line[left_paren..]
        .find(',')
        .ok_or_else(|| PuzzleError::from(PuzzleErrorKind::ParseError))?;

    let right_paren = comma + line[comma..]
        .find(')')
        .ok_or_else(|| PuzzleError::from(PuzzleErrorKind::ParseError))?;

    let src = line[..equals].trim().to_owned();
    let left = line[left_paren+1..comma].trim().to_owned();
    let right = line[comma+1..right_paren].trim().to_owned();

    Ok((src, Destination { left, right }))
}

fn steps_to_first_z(start: &String, instructions: &Vec<Instruction>, map: &HashMap<String,Destination>) -> Result<u64,PuzzleError> {
    let mut steps: u64 = 0;
    let mut current = start;

    let mut instruction_iter = instructions.iter().cycle();

    while ! current.ends_with('Z') {
        let next = map.get(current).ok_or_else(|| PuzzleError::from(PuzzleErrorKind::InputError))?;
        current = match instruction_iter.next().unwrap() {
            Instruction::Left => &next.left,
            Instruction::Right => &next.right
        };
        steps += 1;
    }

    Ok(steps)
}

fn main() -> PuzzleResult<()> {
    let _timer = Timer::new();
    let mut reader = get_puzzle_input()?;

    let mut instructions_line = String::new();
    reader.read_line(&mut instructions_line)?;

    let instructions = instructions_line
        .trim()
        .chars()
        .map(Instruction::try_from)
        .collect::<Result<Vec<Instruction>,PuzzleError>>()?;

    let map: HashMap<String,Destination> = reader
        .lines()
        .map(|x| x.expect("Unable to read input"))
        .filter(|x| x.len() > 0)
        .map(parse_map_line)
        .collect::<Result<HashMap<String,Destination>,_>>()?;

    let mut steps = 0u64;
    let mut instruction_iter = instructions
        .iter()
        .cycle();

    if !cfg!(feature="part2") {
        let mut current_key = &String::from("AAA");

        while current_key != "ZZZ" {
            let destination = map
                .get(current_key)
                .ok_or_else(|| PuzzleError::from(PuzzleErrorKind::InputError))?;

            current_key = match instruction_iter.next().unwrap() {
                Instruction::Left => &destination.left,
                Instruction::Right => &destination.right
            };

            steps += 1;
        }
    }
    else {
        let first_z = map
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(|k| steps_to_first_z(k, &instructions, &map))
            .collect::<Result<Vec<u64>,PuzzleError>>()?;

        assert!(first_z.len() > 0);

        steps = first_z[0];
        for i in &first_z[1..] {
            steps = steps.max(lcm(steps, *i));
        }
    }

    println!("Steps: {}", steps);
    Ok(())
}
