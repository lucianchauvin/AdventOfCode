use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use regex::Regex;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let buf = BufReader::new(f);

    let v = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)")
        .unwrap()
        .captures_iter(
            Regex::new(r"don\'t\(\).*?(do\(\)|$)")
            .unwrap().split(buf.lines().map(|x| x.ok().unwrap())
                .collect::<Vec<String>>()
                .join("").as_str())
            .collect::<Vec<&str>>()
            .join("").as_str())
        .map(|c| c.name("a").unwrap()
            .as_str().parse::<i32>().unwrap()*c.name("b").unwrap()
            .as_str().parse::<i32>().unwrap()).reduce(|i,j| i+j)
        .unwrap();

    println!("{:#?}", v);
    Ok(())
}
