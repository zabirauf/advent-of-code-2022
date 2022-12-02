use std::fs;

pub fn problem1(filename: &str) {
    let file_contents = fs::read_to_string(filename).expect("Error reading file");
    let mut total_scores = 0;

    for line in file_contents.lines() {
        let mut char_line = line.chars();
        let (opponent_move, my_move) = (
            char_line.nth(0).expect("Error char 0") as u32 - 'A' as u32, 
            char_line.nth(1).expect("Error char 2") as u32 - 'X' as u32
        );

        let score = match have_i_won(opponent_move, my_move) {
            RPSResult::Draw => my_move + 1 + 3,
            RPSResult::Lose => my_move + 1,
            RPSResult::Won => my_move + 1 + 6
        };

        total_scores += score;
    }

    println!("Total score is: {}", total_scores);
}

pub fn problem2(filename: &str) {
    let file_contents = fs::read_to_string(filename).expect("Error reading file");
    let mut total_scores = 0;

    for line in file_contents.lines() {
        let mut char_line = line.chars();
        let (opponent_move, outcome) = (
            char_line.nth(0).expect("Error char 0") as u32 - 'A' as u32, 
            char_line.nth(1).expect("Error char 2") as u32 - 'X' as u32
        );

        let score = match outcome {
            0 => get_move(opponent_move, RPSResult::Lose) + 1 + 0,
            1 => opponent_move + 1 + 3,
            2 => get_move(opponent_move, RPSResult::Won) + 1 + 6,
            _ => panic!("Unexpected outcome")
        };

        total_scores += score;
    }

    println!("Total score is: {}", total_scores);

}

enum RPSResult {
    Lose = 0,
    Draw = 1,
    Won = 2
}

fn have_i_won(opponent_move: u32, my_move: u32) -> RPSResult {
    return match (opponent_move, my_move) {
        (o, m) if o == m => RPSResult::Draw,
        (2, 0) => RPSResult::Won,
        (0, 2) => RPSResult::Lose,
        (o, m) if m > o => RPSResult::Won,
        _ => RPSResult::Lose
    }
}

fn get_move(opponent_move: u32, expected_outcome: RPSResult) -> u32 {
    return match (expected_outcome, opponent_move) {
        (RPSResult::Draw, o) => o,
        (RPSResult::Lose, 0) => 2,
        (RPSResult::Lose, o) => o - 1,
        (RPSResult::Won, 2) => 0,
        (RPSResult::Won, o) => o + 1
    };
}