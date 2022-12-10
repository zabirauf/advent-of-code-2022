use std::{fs, collections::HashSet};
use regex::Regex;

pub fn problem1(filename: &str) {
    let re = Regex::new(r"([U|D|L|R]+)\s(\d+)").unwrap();

    let file_contents = fs::read_to_string(filename).expect("Error reading file");
    let mut _unique_pos = HashSet::<String>::new();
    let mut _debug_vec = Vec::<String>::new();

    let mut _head_position = (0, 0);
    let mut _tail_position = (0, 0);

    for line in file_contents.lines() {
        let captures = re.captures(line).expect("Unable to match the line");
        let direction = captures.get(1).unwrap().as_str();
        let moves = captures.get(2).unwrap().as_str().parse::<u8>().unwrap();

        for _ in 0..moves {
            _head_position = move_position(_head_position, direction);
            let (drow, dcol) = distance(_head_position, _tail_position);
            if drow > 1 {
                _tail_position = match (_head_position.0, _tail_position.0) {
                    (hr, tr) if hr > tr => (tr+1, _tail_position.1),
                    (_, tr) => (tr-1, _tail_position.1)
                }
            }
            if dcol > 1 {
                _tail_position = match (_head_position.1, _tail_position.1) {
                    (hc, tc) if hc > tc => (_tail_position.0, tc+1),
                    (_, tc) => (_tail_position.0, tc-1)
                }
            }

            let hash = format!("{}|{}", _tail_position.0, _tail_position.1);
            _unique_pos.insert(hash.clone());
            _debug_vec.push(hash);
        }
    }

    println!("Unique tail positions: {} => {:?}", _unique_pos.len(), _debug_vec);
}

fn distance(head_position: (i32, i32), tail_position: (i32, i32)) -> (u32, u32) {
    return (i32::abs_diff(head_position.0, tail_position.0), i32::abs_diff(head_position.1, tail_position.1))
}

fn move_position(curr_position: (i32, i32), direction: &str) -> (i32, i32) {
    let (row, col) = curr_position;
    return match direction {
        "U" => (row-1, col),
        "D" => (row+1, col),
        "L" => (row, col-1),
        "R" => (row, col+1),
        _ => panic!("Unexpected direction")
    };
}