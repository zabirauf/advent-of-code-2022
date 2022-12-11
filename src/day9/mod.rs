use std::{fs::{self, OpenOptions}, collections::HashSet, io::Write };
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
            _tail_position = get_updated_tail_position(_head_position, _tail_position);

            let hash = format!("{}|{}", _tail_position.0, _tail_position.1);
            _unique_pos.insert(hash.clone());
            _debug_vec.push(hash);
        }
    }

    println!("Unique tail positions: {}", _unique_pos.len());
}

pub fn problem2(filename: &str, output_filename: &str) {
    let re = Regex::new(r"([U|D|L|R]+)\s(\d+)").unwrap();

    let file_contents = fs::read_to_string(filename).expect("Error reading file");
    let mut _unique_pos = HashSet::<(i32, i32)>::new();
    let mut _debug_vec = Vec::<(i32, i32)>::new();

    let mut _positions: [(i32, i32); 10] = [(0,0); 10];
    let last_index = _positions.len()-1;

    for line in file_contents.lines() {
        let captures = re.captures(line).expect("Unable to match the line");
        let direction = captures.get(1).unwrap().as_str();
        let moves = captures.get(2).unwrap().as_str().parse::<u8>().unwrap();

        for _ in 0..moves {
            _positions[0] = move_position(_positions[0], direction);
            for index in 0.._positions.len()-1 {
                _positions[index+1] = get_updated_tail_position(_positions[index], _positions[index+1]);
            }

            let tail_position = _positions[last_index];
            _unique_pos.insert(tail_position);
            _debug_vec.push(tail_position);
        }
    }

    let min_row = _debug_vec.iter().map(|(r, _)| r).min().unwrap().clone();
    let max_row = _debug_vec.iter().map(|(r, _)| r).max().unwrap().clone();
    let min_col = _debug_vec.iter().map(|(_, c)| c).min().unwrap().clone();
    let max_col = _debug_vec.iter().map(|(_, c)| c).max().unwrap().clone();

    let total_rows = max_row - min_row.min(0) + 1;
    let total_cols = max_col - min_col.min(0) + 1;
    let mid_row = min_row.abs();
    let mid_col = min_col.abs();

    let mut grid = Vec::<Vec<char>>::new();
    for _ in 0..total_rows {
        let mut inner = Vec::<char>::new();
        for _ in 0..total_cols {
            inner.push('.');
        }
        grid.push(inner);
    }

    for (pos_r, pos_c) in _debug_vec {
        let r = (pos_r + mid_row) as usize;
        let c = (pos_c + mid_col) as usize;
        grid[r][c] = '#';
    }

    let mut output_file = OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open(output_filename)
        .expect("Unable to create/open output file");

    for cols in grid {
        let line = String::from_iter(cols);
        output_file.write_fmt(format_args!("{}\n", line)).expect("Expected output file to be written to");
    }

    println!("Unique tail positions: {}", _unique_pos.len());
}

fn get_updated_tail_position(head_position: (i32, i32), tail_position: (i32, i32)) -> (i32, i32) {
    let (drow, dcol) = distance(head_position, tail_position);
    if drow > 1 {
        return match (head_position.0, tail_position.0) {
            (hr, tr) if hr > tr => (tr+1, head_position.1),
            (_, tr) => (tr-1, head_position.1)
        }
    }
    else if dcol > 1 {
        return match (head_position.1, tail_position.1) {
            (hc, tc) if hc > tc => (head_position.0, tc+1),
            (_, tc) => (head_position.0, tc-1)
        }
    }

    return tail_position;
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