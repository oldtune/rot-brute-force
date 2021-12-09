#![allow(unused)]
use std::collections::HashMap;

use structopt::StructOpt;
#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref char_map: HashMap<char, u8> = HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
    ]);
}
fn main() {
    let cli = Cli::from_args();
}

fn rot(number: u8, string: &String) {
    let mut result = String::new();

    for (_, c) in string.chars().enumerate() {
        let char_in_map = char_map[c];
    }
}

#[derive(StructOpt)]
struct Cli {
    input: String,
}
