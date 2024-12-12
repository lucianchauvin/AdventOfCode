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

    let v:Vec<Vec<i32>> = buf.lines()
        .map(|x| x.ok().unwrap()
            .split_whitespace().map(|y| y.parse().ok().unwrap())
            .collect::<Vec<i32>>()).collect();

    let r:i32 = (0..v.len())
        .map(|x| (0..v[x].len())
            .map(|y| v[x].clone().into_iter()
                .enumerate().filter(|&(i,_)| i != y)
                .map(|(_, z)| z).collect::<Vec<i32>>().into_iter().collect::<Vec<i32>>()
                .is_monotone_by()).reduce(|a,b| a || b).unwrap() as i32)
        .sum();

    println!("{:#?}", r);
    Ok(())
}
