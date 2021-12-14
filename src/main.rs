#![allow(unused)]
use std::collections::HashMap;

use structopt::StructOpt;

fn main() {
    let cli = Cli::from_args();
    for i in 1..14 {
        println!("{}", rot(i, &cli.input));
    }
}

fn rot(number: u8, string: &String) -> String {
    let mut result = String::new();

    for c in string.chars() {
        let code_point = c as u8;
        let mutated_char: u8;
        if code_point >= 97 && code_point <= 122 {
            mutated_char = (code_point + number) % 97 % 26 + 97;
        } else if (code_point >= 65 && code_point <= 90) {
            mutated_char = (code_point + number) % 65 % 26 + 65;
        } else {
            mutated_char = code_point;
        }
        result.push(mutated_char as char);
    }

    result
}

#[derive(StructOpt)]
struct Cli {
    input: String,
}
