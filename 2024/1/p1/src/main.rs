use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use itertools::Itertools;
use itertools::sorted as sorted;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let buf = BufReader::new(f);

    let (l,r):(Vec<i32>, Vec<i32>) = buf.lines()
        .map(|x| x.ok().unwrap().split_whitespace().map(|y| y.parse().ok().unwrap())
        .into_iter().next_tuple::<(i32, i32)>().unwrap()).unzip();

    println!("{}", (0..(l.len()))
        .map(|i| (sorted(l.clone().into_iter()).nth(i).unwrap() - sorted(r.clone().into_iter()).nth(i).unwrap()).abs())
        .reduce(|x,y| x + y).unwrap());

    Ok(())
}
