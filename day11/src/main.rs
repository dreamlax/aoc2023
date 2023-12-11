use std::collections::BTreeSet;
use std::io::BufRead;
use aoc_utils::prelude::*;

#[derive(Debug)]
pub struct Galaxy {
    x: i64,
    y: i64,
}

impl Galaxy {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Self) -> i64 {
        let (fx, gx) = (self.x.min(other.x), self.x.max(other.x));
        let (fy, gy) = (self.y.min(other.y), self.y.max(other.y));
        let dx = gx - fx;
        let dy = gy - fy;
        dx + dy
    }
}

#[derive(Debug)]
pub struct Universe {
    galaxies: Vec<Galaxy>
}

impl Universe {
    pub fn new() -> Self {
        Self { galaxies: Vec::new() }
    }

    pub fn expand_x(&mut self, filled_columns: &BTreeSet<i64>) {
        let maximum = *filled_columns.last().unwrap() + 1;
        let mut offsets: Vec<i64> = Vec::new();

        let mut current_offset = 0;

        for i in 0..maximum {
            if !filled_columns.contains(&i) {
                if cfg!(feature="part2") {
                    current_offset += 999_999;
                } else {
                    current_offset += 1;
                }
            }
            offsets.push(current_offset);
        }

        for g in &mut self.galaxies {
            g.x += offsets[g.x as usize];
        }
    }
}

fn main() -> PuzzleResult<()> {
    let _timer = Timer::new();
    let reader = get_puzzle_input()?;

    let mut line_counter = 0;
    let mut universe = Universe::new();
    let mut filled_columns = BTreeSet::new();

    for line in reader.lines() {
        let line = line?;

        let new_galaxies = line
            .chars()
            .enumerate()
            .filter_map(|(idx,ch)|
                if ch == '#' {
                    Some(Galaxy::new(idx as i64, line_counter as i64))
                }
                else {
                    None
                }
            )
            .collect::<Vec<Galaxy>>();

        filled_columns.extend(new_galaxies
            .iter()
            .map(|g| g.x)
        );

        line_counter += if new_galaxies.is_empty() {
                if cfg!(feature="part2") {
                    1_000_000
                } else {
                    2
                }
            }
            else {
                1
            };
        universe.galaxies.extend(new_galaxies);
    }

    universe.expand_x(&filled_columns);

    let mut distances: i64 = 0;
    
    for i in 0..universe.galaxies.len() - 1 {
        for j in i+1..universe.galaxies.len() {
            let g1 = &universe.galaxies[i];
            let g2 = &universe.galaxies[j];
            let distance = g1.distance(&g2);

            #[cfg(debug_assertions)]
            println!("{:?} ==> {:?} = {}", g1, g2, distance);

            distances += distance;
        }
    }

    println!("Distances: {}", distances);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_1() {
        let g1 = Galaxy::new(1, 6);
        let g2 = Galaxy::new(5, 11);

        let distance = g1.distance(&g2);

        assert_eq!(distance, 9, "distance should be 9");
    }

    #[test]
    fn test_distance_2() {
        let g1 = Galaxy::new(4, 0);
        let g2 = Galaxy::new(9, 10);

        let distance = g1.distance(&g2);

        assert_eq!(distance, 15, "distance should be 15");
    }

    #[test]
    fn test_distance_3() {
        let g1 = Galaxy::new(0, 11);
        let g2 = Galaxy::new(5, 11);

        let distance = g1.distance(&g2);

        assert_eq!(distance, 5, "distance should be 5");
    }

    #[test]
    fn test_distance_4() {
        let g1 = Galaxy::new(5, 0);
        let g2 = Galaxy::new(10, 1);

        let distance = g1.distance(&g2);

        assert_eq!(distance, 6, "distance should be 6");
    }
}
