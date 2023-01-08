use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let line_break = String::from("");

    let mut numbers: Vec<u32> = Vec::new();
    let mut current_number = 0u32;

    let lines = parse_input("input.txt").expect("Could not read file!");
    for line in lines {
        match line {
            Ok(line) => {
                if line == line_break {
                    numbers.push(current_number);
                    current_number = 0;
                } else {
                    current_number += line.parse::<u32>().unwrap();
                }
            }
            Err(_) => panic!("Could not read line!"),
        };
    }

    numbers.sort_unstable();

    println!(
        "The answer is: {:?}",
        numbers.iter().rev().take(3).sum::<u32>()
    );
}
