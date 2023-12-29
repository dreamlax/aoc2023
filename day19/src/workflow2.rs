use std::collections::HashMap;
use std::ops::Range;
use aoc_utils::rangeutils::Intersect;

use aoc_utils::error::{PuzzleErrorKind, PuzzleError};
use aoc_utils::result::PuzzleResult;

use crate::condition::ConditionResult2;

type Condition2<'a> = dyn Fn([Range<u32>; 4]) -> ConditionResult2<'a> + 'a;

fn parse_condition_part2<'a>(expression: &'a str) -> PuzzleResult<Box<Condition2<'a>>> {
    let op = expression
        .find(|ch| matches!(ch, '<' | '>'));

    let Some(op) = op else {
        return Ok(Box::new(|ranges| ConditionResult2::FallThrough(expression, ranges)));
    };

    let colon = op + expression[op..]
        .find(|ch| ch == ':')
        .ok_or_else(|| PuzzleErrorKind::ParseError)?;

    let range_index = match &expression[0..op] {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => panic!("Weird rule...")
    };
    let value = u32::from_str_radix(&expression[op+1..colon], 10)?;
    let next = &expression[colon+1..];

    let op = &expression[op..=op];
    let (this_range, opposite_range) = match op {
        "<" => (1..value, value..4001),
        ">" => (value+1..4001, 1..value+1),
        _ => return Err(PuzzleErrorKind::InputError.into())
    };

    Ok(Box::new(move |mut ranges| {
        let mut remainder = ranges.clone();
        ranges[range_index] = ranges[range_index].intersect(&this_range).unwrap_or(0..0);
        remainder[range_index] = remainder[range_index].intersect(&opposite_range).unwrap_or(0..0);
        ConditionResult2::Next(next, ranges, remainder)
    }))
}

pub struct WorkflowDB<'a> {
    functions: HashMap<&'a str, Vec<Box<Condition2<'a>>>>,
}

impl<'a> WorkflowDB<'a> {
    pub fn new() -> Self {
        let mut wdb = Self {
            functions: HashMap::new()
        };

        wdb.functions.insert("A", vec![Box::new(|ranges| ConditionResult2::Approved(ranges))]);
        wdb.functions.insert("R", vec![Box::new(|ranges| ConditionResult2::Rejected(ranges))]);

        wdb
    }

    pub fn add_workflow(&mut self, name: &'a str, expressions: &'a str) -> PuzzleResult<()> {
        let conditions = expressions
            .split(',')
            .map(parse_condition_part2)
            .collect::<Result<Vec<_>,_>>()?;

        self.functions.insert(name, conditions);

        Ok(())
    }

    pub fn total_parts(&self) -> u64 {
        let mut approved = 0;
        let mut rejected = 0;

        let mut continuations = Vec::new();

        continuations.push(("in", [1..4001, 1..4001, 1..4001, 1..4001]));

        while let Some((workflow_name, mut main_ranges)) = continuations.pop() {
            let conditions = self.functions.get(workflow_name).expect("Missing workflow");

            for condition in conditions {
                match condition(main_ranges.clone()) {
                    ConditionResult2::Approved(ranges) => {
                        approved += ranges.iter().map(|r| r.len() as u64).product::<u64>();
                    },
                    ConditionResult2::Rejected(ranges) => {
                        rejected += ranges.iter().map(|r| r.len() as u64).product::<u64>();
                    },
                    ConditionResult2::Next(next, ranges, remainder) => {
                        continuations.push((next, ranges));
                        main_ranges = remainder;
                    },
                    ConditionResult2::FallThrough(next, ranges) => {
                        continuations.push((next, ranges.clone()));
                    }
                }
            }
        }


        dbg!(256_000_000_000_000 - rejected);
        approved
    }
}
