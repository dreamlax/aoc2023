use std::io::BufRead;
use aoc_utils::prelude::*;

mod directive;
use directive::Directive;

mod point;
use point::Point;

fn main() -> PuzzleResult<()> {
    let _timer = Timer::new();
    let input = get_puzzle_input()?;
    let lines = input.lines();

    let directives = lines
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .map(Directive::try_from)
        .collect::<Result<Vec<_>,_>>()
        .expect("Unable to collect directives");

    let mut absolute = Vec::new();
    absolute.push(Point { x: 0, y: 0 });
    let absolute = directives
        .iter()
        .fold((absolute, 0isize, 0isize), |(mut result, dx, dy), d| {
            let dx = dx + d.dx;
            let dy = dy + d.dy;
            result.push((dx, dy).into());
            (result, dx, dy)
        }).0;

    let answer: isize = absolute
        .windows(2)
        .map(|points| points[0].x * points[1].y - points[1].x * points[0].y + (points[1].x - points[0].x + points[1].y - points[0].y).abs())
        .sum::<isize>() / 2 + 1;

    println!("{:?}", answer);

    Ok(())
}
