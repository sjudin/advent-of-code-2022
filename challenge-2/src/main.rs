use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq, Eq)]
enum PlayerOption {
    Rock,
    Paper,
    Scissors,
}

enum GameOutcome {
    Win,
    Loss,
    Tie,
}

struct GameResult {
    result: GameOutcome,
    score: u32,
}

fn check_player_option(opt_1: PlayerOption, opt_2: PlayerOption) -> GameResult {
    // opt_1 is the opponents choice and opt_2 is the players choice

    let tie_score = 3;
    let win_score = 6;
    let rock_score = 1;
    let paper_score = 2;
    let scissor_score = 3;

    // Handle the case where both players have the same hand
    if opt_1 == opt_2 {
        match opt_2 {
            PlayerOption::Rock => {
                return GameResult {
                    result: GameOutcome::Tie,
                    score: tie_score + rock_score,
                }
            }
            PlayerOption::Paper => {
                return GameResult {
                    result: GameOutcome::Tie,
                    score: tie_score + paper_score,
                }
            }
            PlayerOption::Scissors => {
                return GameResult {
                    result: GameOutcome::Tie,
                    score: tie_score + scissor_score,
                }
            }
        };
    }

    match opt_2 {
        PlayerOption::Rock => {
            if opt_1 == PlayerOption::Scissors {
                GameResult {
                    result: GameOutcome::Win,
                    score: win_score + rock_score,
                }
            } else {
                GameResult {
                    result: GameOutcome::Loss,
                    score: rock_score,
                }
            }
        }
        PlayerOption::Paper => {
            if opt_1 == PlayerOption::Rock {
                GameResult {
                    result: GameOutcome::Win,
                    score: win_score + paper_score,
                }
            } else {
                GameResult {
                    result: GameOutcome::Loss,
                    score: paper_score,
                }
            }
        }
        PlayerOption::Scissors => {
            if opt_1 == PlayerOption::Paper {
                GameResult {
                    result: GameOutcome::Win,
                    score: win_score + scissor_score,
                }
            } else {
                GameResult {
                    result: GameOutcome::Loss,
                    score: scissor_score,
                }
            }
        }
    }
}

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn player_option_from_char(c: &char) -> Result<PlayerOption, &str> {
    match c {
        'A' | 'X' => Ok(PlayerOption::Rock),
        'B' | 'Y' => Ok(PlayerOption::Paper),
        'C' | 'Z' => Ok(PlayerOption::Scissors),
        _ => Err("Invalid option!"),
    }
}

fn parse_input(line: &str, is_part_one: bool) -> Result<(PlayerOption, PlayerOption), &str> {
    if line.len() != 3 {
        return Err("Line length must be 3!");
    };
    let binding_opt_1 = line.chars().nth(0).unwrap();
    let binding_opt_2 = line.chars().nth(2).unwrap();

    let opt_1 = player_option_from_char(&binding_opt_1).unwrap();
    let opt_2 = player_option_from_char(&binding_opt_2).unwrap();

    if is_part_one {
        // Solution to first problem
        return Ok((opt_1, opt_2));
    } else {
        match (binding_opt_1, binding_opt_2) {
            ('A', 'X') => return Ok((PlayerOption::Rock, PlayerOption::Scissors)),
            ('A', 'Y') => return Ok((PlayerOption::Rock, PlayerOption::Rock)),
            ('A', 'Z') => return Ok((PlayerOption::Rock, PlayerOption::Paper)),

            ('B', 'X') => return Ok((PlayerOption::Paper, PlayerOption::Rock)),
            ('B', 'Y') => return Ok((PlayerOption::Paper, PlayerOption::Paper)),
            ('B', 'Z') => return Ok((PlayerOption::Paper, PlayerOption::Scissors)),

            ('C', 'X') => return Ok((PlayerOption::Scissors, PlayerOption::Paper)),
            ('C', 'Y') => return Ok((PlayerOption::Scissors, PlayerOption::Scissors)),
            ('C', 'Z') => return Ok((PlayerOption::Scissors, PlayerOption::Rock)),
            _ => panic!("Invalid input!"),
        }
    }
}

fn main() {
    // Set to false to run the answer for part 2
    let is_part_one = false;

    let mut score: u32 = 0;

    let lines = read_input("input.txt").expect("Could not read file!");
    for line in lines {
        match line {
            Ok(line) => match parse_input(&line, is_part_one) {
                Ok((opt_1, opt_2)) => match check_player_option(opt_1, opt_2) {
                    state => {
                        score += state.score;
                    }
                },
                Err(err) => panic!("{err}"),
            },
            Err(_) => panic!("Could not read line!"),
        }
    }

    println!("The answer is: {score}")
}
