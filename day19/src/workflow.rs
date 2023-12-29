use std::collections::HashMap;

use aoc_utils::error::PuzzleErrorKind;
use aoc_utils::result::PuzzleResult;

use crate::condition::ConditionResult;
use crate::part::Part;

type Condition<'a, 'b> = dyn Fn(&'b Part) -> ConditionResult<'a> + 'a;

pub fn parse_condition<'a, 'b>(expression: &'a str) -> PuzzleResult<Box<Condition<'a, 'b>>> {
    let op = expression
        .find(|ch| matches!(ch, '<' | '>'));

    let Some(op) = op else {
        return Ok(Box::new(|_| ConditionResult::Next(expression)));    
    };

    let colon = op + expression[op..]
        .find(|ch| ch == ':')
        .ok_or_else(|| PuzzleErrorKind::ParseError)?;

    let field = &expression[0..op];
    let value = u32::from_str_radix(&expression[op+1..colon], 10)?;
    let result = ConditionResult::Next(&expression[colon+1..]);

    let op = &expression[op..=op];
    let func = match op {
        "<" => u32::lt,
        ">" => u32::gt,
        _ => return Err(PuzzleErrorKind::InputError.into())
    };

    Ok(Box::new(move |part: &'b Part| {
        let part_field = match field {
            "x" => part.x,
            "m" => part.m,
            "a" => part.a,
            "s" => part.s,
            _ => panic!("Weird rule ...")
        };

        if func(&part_field, &value) {
            result
        }
        else {
            ConditionResult::Nothing
        }
    }))
}

pub struct WorkflowDB<'a, 'b> {
    functions: HashMap<&'a str, Vec<Box<Condition<'a, 'b>>>>,
}

impl<'a, 'b> WorkflowDB<'a, 'b> {
    pub fn new() -> Self {
        let mut wdb = Self {
            functions: HashMap::new()
        };

        wdb.functions.insert("A", vec![Box::new(|_: &'b Part| ConditionResult::Approved)]);
        wdb.functions.insert("R", vec![Box::new(|_: &'b Part| ConditionResult::Rejected)]);

        wdb
    }

    pub fn add_workflow(&mut self, name: &'a str, expressions: &'a str) -> PuzzleResult<()> {
        let conditions = expressions
            .split(',')
            .map(parse_condition)
            .collect::<Result<Vec<_>,_>>()?;

        self.functions.insert(name, conditions);

        Ok(())
    }

    pub fn part_value(&self, part: &'b Part) -> u32 {
        let mut conditions = self.functions.get("in").expect("Missing workflow named 'in'");

        loop {
            for condition in conditions {
                match condition(part) {
                    ConditionResult::Nothing => (),
                    ConditionResult::Approved => return part.value(),
                    ConditionResult::Rejected => return 0,
                    ConditionResult::Next(next) => {
                        conditions = self.functions.get(next).expect("Missing workflow");
                        break;
                    }
                }
            }
        }
    }
}
