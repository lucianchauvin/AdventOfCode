use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use regex::Regex;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let buf = BufReader::new(f);

    let v = buf.lines()
        .map(|x| Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)")
            .unwrap()
            .captures_iter(x.ok().unwrap().as_str())
            .map(|c| c.name("a").unwrap()
                .as_str().parse::<i32>().unwrap()*c.name("b").unwrap()
                .as_str().parse::<i32>().unwrap()).reduce(|i,j| i+j))
        .reduce(|i,j| Some(i.unwrap()+j.unwrap())).unwrap().unwrap();

    println!("{:#?}", v);
    Ok(())
}
