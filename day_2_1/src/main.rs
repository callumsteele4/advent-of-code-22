use std::fs;

#[derive(Copy, Clone)]
enum Action {
    Unknown = 0,
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Result {
    Loss = 0,
    Draw = 3,
    Victory = 6,
}

fn determine_result(your_action: Action, opponent_action: Action) -> Result {
    match (your_action, opponent_action) {
        (your_action, opponent_action)
            if your_action as i32 % 3 == (opponent_action as i32 + 1) % 3 =>
        {
            Result::Victory
        }
        (your_action, opponent_action) if your_action as i32 == opponent_action as i32 => {
            Result::Draw
        }
        _ => Result::Loss,
    }
}

fn calculate_score(your_action: Action, opponent_action: Action) -> i32 {
    let mut score = 0;
    score += your_action as i32;
    score += determine_result(your_action, opponent_action) as i32;
    score
}

fn parse_action(input: &str) -> Action {
    match input {
        "A" | "X" => Action::Rock,
        "B" | "Y" => Action::Paper,
        "C" | "Z" => Action::Scissors,
        _ => Action::Unknown,
    }
}

fn main() {
    let rounds =
        fs::read_to_string("test_file.txt").expect("Should have been able to read the file");

    let mut scores = vec![];

    for round in rounds.split("\n") {
        let actions = round.split(" ").collect::<Vec<_>>();
        scores.push(calculate_score(
            parse_action(actions[1]),
            parse_action(actions[0]),
        ));
    }

    println!("{:?}", scores.iter().sum::<i32>());
}
