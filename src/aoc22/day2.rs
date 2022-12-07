use crate::utils;
use crate::utils::printAnswer;

#[derive(Clone, Copy)]
enum Guess {
    Rock,
    Paper,
    Scissors
}

impl Guess {
    fn from_opp(s: &str) -> Guess {
        match s {
            "A" => Guess::Rock,
            "B" => Guess::Paper,
            "C" => Guess::Scissors,
            _ => panic!("opponent guess must be one of A, B, or C!")
        }
    }

    fn from_mine(s: &str) -> Guess {
        match s {
            "X" => Guess::Rock,
            "Y" => Guess::Paper,
            "Z" => Guess::Scissors,
            _ => panic!("my guess must be one of X, Y, or Z!")
        }
    }
}

#[derive(Clone, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win
}

impl Outcome {
    fn from(s: &str) -> Outcome {
        match s {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("outcome must be one of X, Y, or Z!")
        }
    }
}

fn compute_guess(opponent: Guess, outcome: Outcome) -> Guess {
    match outcome {
        Outcome::Lose => {
            match opponent {
                Guess::Rock => Guess::Scissors,
                Guess::Paper => Guess::Rock,
                Guess::Scissors => Guess::Paper
            }
        }
        Outcome::Draw => opponent,
        Outcome::Win => {
            match opponent {
                Guess::Rock => Guess::Paper,
                Guess::Paper => Guess::Scissors,
                Guess::Scissors => Guess::Rock
            }
        }
    }
}

fn compute_score(opponent: Guess, mine: Guess) -> i32 {
    let mut total = 0;
    match mine {
        Guess::Rock => {
            // Part 2 = lose
            total += 1;
            match opponent {
                Guess::Rock => total += 3,
                Guess::Paper => total += 0,
                Guess::Scissors => total += 6
            }
        }
        Guess::Paper => {
            // Part 2 = draw
            total += 2;
            match opponent {
                Guess::Rock => total += 6,
                Guess::Paper => total += 3,
                Guess::Scissors => total += 0
            }
        }
        Guess::Scissors => {
            // Part 2 = win
            total += 3;
            match opponent {
                Guess::Rock => total += 0,
                Guess::Paper => total += 6,
                Guess::Scissors => total += 3
            }
        }
    }
    total
}

pub fn solve() {
    part_1();
    part_2();
}

fn part_1() {
    let input_as_lines = utils::get_input_as_lines("input/aoc22/day2.txt");

    let mut score = 0;
    for line in input_as_lines {
        let them = Guess::from_opp(line.split(" ").nth(0).unwrap());
        let mine = Guess::from_mine(line.split(" ").nth(1).unwrap());
        score += compute_score(them, mine);
    }

    let part_1_answer = format!("When the strategy is perfect, my score is {}", score);
    printAnswer(2, 1, part_1_answer.as_str())
}

fn part_2() {
    let input_as_lines = utils::get_input_as_lines("input/aoc22/day2.txt");
    let mut score = 0;
    for line in input_as_lines {
        let them = Guess::from_opp(line.split(" ").nth(0).unwrap());
        let outcome = Outcome::from(line.split(" ").nth(1).unwrap());
        let mine = compute_guess(them, outcome);
        score += compute_score(them, mine);
    }

    let part_2_answer = format!("By following the encrypted strategy, my score is {}", score);
    printAnswer(2, 2, part_2_answer.as_str())
}