use std::fs;

#[derive(Copy, Clone)]
enum Action {
    Unknown = 0,
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Copy, Clone)]
enum Result {
    Unknown = -1,
    Loss = 0,
    Draw = 3,
    Victory = 6,
}

fn parse_action_from_int(input: i32) -> Action {
    match input {
        1 => Action::Rock,
        2 => Action::Paper,
        3 => Action::Scissors,
        _ => Action::Unknown,
    }
}

fn determine_action(opponent_action: Action, result: Result) -> Action {
    match result {
        Result::Victory => parse_action_from_int(opponent_action as i32 % 3 + 1),
        Result::Loss => parse_action_from_int((opponent_action as i32 + 1) % 3 + 1),
        _ => opponent_action,
    }
}

fn calculate_score(opponent_action: Action, result: Result) -> i32 {
    let mut score = 0;
    score += result as i32;
    score += determine_action(opponent_action, result) as i32;
    score
}

fn parse_action_from_string(input: &str) -> Action {
    match input {
        "A" => Action::Rock,
        "B" => Action::Paper,
        "C" => Action::Scissors,
        _ => Action::Unknown,
    }
}

fn parse_result(input: &str) -> Result {
    match input {
        "X" => Result::Loss,
        "Y" => Result::Draw,
        "Z" => Result::Victory,
        _ => Result::Unknown,
    }
}

fn main() {
    let rounds =
        fs::read_to_string("test_file.txt").expect("Should have been able to read the file");

    let mut scores = vec![];

    for round in rounds.split("\n") {
        let actions = round.split(" ").collect::<Vec<_>>();
        scores.push(calculate_score(
            parse_action_from_string(actions[0]),
            parse_result(actions[1]),
        ));
    }

    println!("{:?}", scores.iter().sum::<i32>());
}
