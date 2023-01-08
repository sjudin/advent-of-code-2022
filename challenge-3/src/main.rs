use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calculate_priority(c: &char) -> u8 {
    if c.is_ascii_uppercase() {
        return c.to_owned() as u8 - 38;
    } else {
        return c.to_owned() as u8 - 96;
    }
}

fn part_one() {
    let mut ans: u32 = 0;
    let lines = read_input("input.txt").expect("Could not read file!");

    for line in lines {
        match line {
            Ok(line) => {
                let first: HashSet<char> = HashSet::from_iter(line.chars().take(line.len() / 2));
                let second: HashSet<char> =
                    HashSet::from_iter(line.chars().rev().take(line.len() / 2));

                ans += calculate_priority(first.intersection(&second).into_iter().nth(0).unwrap())
                    as u32;
            }
            Err(_) => panic!("Could not read line!"),
        }
    }
    println!("The answer is {ans}")
}

fn get_strings(lines: &mut std::io::Lines<BufReader<File>>) -> Option<(String, String, String)> {
    let l1 = match lines.next() {
        Some(s) => s.unwrap(),
        None => return None,
    };

    Some((
        l1,
        lines.next().unwrap().unwrap(),
        lines.next().unwrap().unwrap(),
    ))
}

fn part_two() {
    let mut ans: u32 = 0;
    let mut lines = read_input("input.txt").expect("Could not read file!");

    while let Some(group) = get_strings(&mut lines) {
        // Split group into three sets
        let first: HashSet<char> = HashSet::from_iter(group.0.chars());
        let second: HashSet<char> = HashSet::from_iter(group.1.chars());
        let third: HashSet<char> = HashSet::from_iter(group.2.chars());

        // Find the intersection between all three sets
        let i = first
            .intersection(&second)
            .copied()
            .collect::<HashSet<char>>();

        ans += calculate_priority(&i.intersection(&third).nth(0).unwrap()) as u32;
    }
    println!("The answer is {ans}")
}

fn main() {
    part_one();
    part_two();
}
