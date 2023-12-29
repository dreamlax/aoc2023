use std::io::Read;
use aoc_utils::prelude::*;

mod condition;
mod part;
mod workflow;
mod workflow2;

use workflow::WorkflowDB;
use workflow2::WorkflowDB as WorkflowDB2;

fn main() -> PuzzleResult<()> {
    let _timer = Timer::new();

    let mut input = String::new();
    get_puzzle_input()?.read_to_string(&mut input)?;
    
    let (workflows, parts) = input.split_once("\n\n").expect("Unable to parse input, missing double blank");

    if cfg!(not(feature = "part2")) {
        let parts = parts
            .lines()
            .map(part::Part::try_from)
            .collect::<Result<Vec<_>,_>>()
            .map_err(|_| PuzzleErrorKind::InputError)?;

        let workflowdb = workflows
            .lines()
            .fold(WorkflowDB::new(), |mut db, line| {
                let open_curly = line.find(|ch| ch == '{').expect("No open curly");
                let close_curly = open_curly + line[open_curly..].find(|ch| ch == '}').expect("No close curly");

                let name = &line[..open_curly];
                let expressions = &line[open_curly+1..close_curly];
                db.add_workflow(name, expressions).expect(&format!("Unable to parse expressions: {expressions}"));
                db
            });

        let answer: u32 = parts
            .iter()
            .map(|p| workflowdb.part_value(p))
            .sum();

        println!("Answer: {answer}");
    }
    else {
        let workflowdb = workflows
            .lines()
            .fold(WorkflowDB2::new(), |mut db, line| {
                let open_curly = line.find(|ch| ch == '{').expect("No open curly");
                let close_curly = open_curly + line[open_curly..].find(|ch| ch == '}').expect("No close curly");

                let name = &line[..open_curly];
                let expressions = &line[open_curly+1..close_curly];
                db.add_workflow(name, expressions).expect(&format!("Unable to parse expressions: {expressions}"));
                db
            });
        
        let answer: u64 = workflowdb.total_parts();

        println!("Answer: {answer}");
    }

    Ok(())
}
