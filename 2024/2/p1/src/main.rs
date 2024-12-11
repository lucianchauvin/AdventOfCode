use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
// use itertools::Itertools;
// use itertools::sorted as sorted;

trait MonotoneBy {
    fn is_monotone_by(&self) -> bool;
}

impl MonotoneBy for Vec<i32> {
    fn is_monotone_by(&self) -> bool {
        if self.is_empty() {
            return true;
        }

        let mut a = true;
        let mut b = true;

        for i in 1..self.len() {
            let c = (self[i-1] - self[i]).abs();
            if c < 1 || c > 3 {
                return false;
            }
            if self[i-1] < self[i] {
                a = false;
            }
            if self[i-1] > self[i] {
                b = false;
            }
        }

        a || b
    }
}

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let buf = BufReader::new(f);

    let v:i32 = buf.lines()
        .map(|x| x.ok().unwrap()
            .split_whitespace().map(|y| y.parse().ok().unwrap())
            .collect::<Vec<i32>>().is_monotone_by() as i32).sum();

    println!("{:#?}", v);
    Ok(())
}
