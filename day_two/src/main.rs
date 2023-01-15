use std::fs;

#[derive(Debug, PartialEq)]
enum Action {
    Rock,
    Paper,
    Scissors,
}
enum Outcome {
    Win,
    Loss,
    Draw,
}

const SCORE_WIN: i32 = 6;
const SCORE_DRAW: i32 = 3;
const SCORE_LOSS: i32 = 0;

const SCORE_ROCK: i32 = 1;
const SCORE_PAPER: i32 = 2;
const SCORE_SCISSORS: i32 = 3;

fn main() {
    let raw_games = fs::read_to_string("./input.txt").unwrap();
    let stage_one_games = process_games(&raw_games);
    determine_summary(stage_one_games);

    let stage_two_games = process_game_two(&raw_games);
    determine_summary(stage_two_games);
}

fn process_game_two(raw: &str) -> Vec<(Action, Action)> {
    let mut games: Vec<(Action, Action)> = Vec::new();

    for raw_game in raw.lines() {
        let opponent_move = raw_game.chars().nth(0);
        let player_move = raw_game.chars().nth(2);

        let game = match opponent_move {
            Some('A') =>
                match player_move {
                    Some('X') => (Action::Rock, Action::Scissors),
                    Some('Y') => (Action::Rock, Action::Rock),
                    Some('Z') => (Action::Rock, Action::Paper),
                    _ => {
                        continue;
                    }
                }
            Some('B') =>
                match player_move {
                    Some('X') => (Action::Paper, Action::Rock),
                    Some('Y') => (Action::Paper, Action::Paper),
                    Some('Z') => (Action::Paper, Action::Scissors),
                    _ => {
                        continue;
                    }
                }
            Some('C') =>
                match player_move {
                    Some('X') => (Action::Scissors, Action::Paper),
                    Some('Y') => (Action::Scissors, Action::Scissors),
                    Some('Z') => (Action::Scissors, Action::Rock),
                    _ => {
                        continue;
                    }
                }
            _ => {
                continue;
            }
        };

        games.push(game);
    }

    games
}

#[test]
fn test_process_game_two() {
    let raw_games = "\
A X
A Y
A Z
B X
B Y
B Z
C X
C Y
C Z
";
    let expected_games = vec![
        (Action::Rock, Action::Scissors), // 0 + 3
        (Action::Rock, Action::Rock), // 3 + 1
        (Action::Rock, Action::Paper), // 6 + 2
        (Action::Paper, Action::Rock), // 0 + 2
        (Action::Paper, Action::Paper), // 3 + 2
        (Action::Paper, Action::Scissors), // 6 + 3
        (Action::Scissors, Action::Paper), // 0 + 2
        (Action::Scissors, Action::Scissors), // 3 + 3
        (Action::Scissors, Action::Rock) // 6 + 1
    ];
    let games = process_game_two(raw_games);
    assert_eq!(games, expected_games);

    assert_eq!(determine_summary(games), 45)
}

fn process_games(raw: &str) -> Vec<(Action, Action)> {
    let mut games: Vec<(Action, Action)> = Vec::new();

    for raw_game in raw.lines() {
        let opponent = match raw_game.chars().nth(0) {
            Some('A') => Action::Rock,
            Some('B') => Action::Paper,
            Some('C') => Action::Scissors,
            _ => {
                continue;
            }
        };
        let player = match raw_game.chars().nth(2) {
            Some('X') => Action::Rock,
            Some('Y') => Action::Paper,
            Some('Z') => Action::Scissors,
            _ => {
                continue;
            }
        };
        games.push((opponent, player));
    }

    games
}

fn game_outcome(opponent_move: Action, player_move: Action) -> i32 {
    // Points for choosing a particular action
    let choice_score = match player_move {
        Action::Rock => SCORE_ROCK,
        Action::Paper => SCORE_PAPER,
        Action::Scissors => SCORE_SCISSORS,
    };

    let round_outcome = match determine_winner(&opponent_move, &player_move) {
        Outcome::Win => SCORE_WIN,
        Outcome::Draw => SCORE_DRAW,
        Outcome::Loss => SCORE_LOSS,
    };

    choice_score + round_outcome
}

fn determine_winner(opponent: &Action, player: &Action) -> Outcome {
    match opponent {
        Action::Rock =>
            match player {
                Action::Scissors => Outcome::Loss,
                Action::Paper => Outcome::Win,
                _ => Outcome::Draw,
            }
        Action::Paper =>
            match player {
                Action::Rock => Outcome::Loss,
                Action::Scissors => Outcome::Win,
                _ => Outcome::Draw,
            }
        Action::Scissors =>
            match player {
                Action::Paper => Outcome::Loss,
                Action::Rock => Outcome::Win,
                _ => Outcome::Draw,
            }
    }
}

fn determine_summary(games: Vec<(Action, Action)>) -> i32 {
    let mut total_score = 0;
    for (opponent_move, player_move) in games {
        let score = game_outcome(opponent_move, player_move);
        total_score += score;
    }
    println!("Total score: {total_score}");
    total_score
}