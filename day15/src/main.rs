use std::io::Read;
use aoc_utils::prelude::*;

struct HashMap<'a> {
    pub buckets: Vec<Vec<(&'a [u8], u8)>>
}

impl<'a> HashMap<'a> {
    fn new() -> Self {
        let mut v = Vec::new();
        for _ in 0..256 {
            v.push(Vec::new());
        }
        Self {
            buckets: v
        }
    }

    fn add(&mut self, key: &'a [u8], value: u8) {
        let h: usize = key.hash().into();

        for entry in self.buckets[h].iter_mut() {
            if entry.0 == key {
                entry.1 = value;
                return;
            }
        }

        self.buckets[h].push((key, value))
    }

    fn del(&mut self, key: &'a [u8]) {
        let h: usize = key.hash().into();

        if let Some(index) = self.buckets[h].iter().position(|(k, _)| *k == key) {
            self.buckets[h].remove(index);
        }
    }

    fn focal_power(&self) -> u64 {
        self.buckets
            .iter()
            .enumerate()
            .flat_map(|(bucket_index, bucket)| bucket
                .iter()
                .enumerate()
                .map(move |(lens_index,(_name, power))| ((bucket_index + 1) * (lens_index + 1) * *power as usize) as u64)
            )
            .sum()
    }
}

trait AdventHash<T> {
    fn hash(&self) -> T;
}

impl<> AdventHash<u8> for &[u8] {
    fn hash(&self) -> u8 {
        self
            .iter()
            .fold(0u8, |hash, &ch| if ch == b'\n' {
                    hash
                }
                else {
                    hash
                        .wrapping_add(ch)
                        .wrapping_mul(17)
                }
            )
    }
}

fn main() -> PuzzleResult<()> {
    let _timer = Timer::new();
    
    let mut buf = Vec::new();
    get_puzzle_input()?.read_to_end(&mut buf)?;

    if cfg!(not(feature = "part2")) {
        let hash: u32 = buf
            .split(|b| *b == b',')
            .map(|seq| seq.hash() as u32)
            .sum();

        println!("Answer: {}", hash);
    }
    else {
        let result = buf
            .split(|b| *b == b',')
            .fold(HashMap::new(), |mut hashmap, segment| {
                let operation = segment
                    .iter()
                    .position(|p| matches!(p, b'-' | b'='))
                    .expect("Input error!");

                match segment[operation] {
                    b'=' => hashmap.add(&segment[0..operation], segment[operation+1] - b'0'),
                    b'-' => hashmap.del(&segment[0..operation]),
                    _ => panic!("Shouldn't happen...")
                }

                hashmap
            });

        println!("Answer: {}", result.focal_power());
    }

    Ok(())
}
