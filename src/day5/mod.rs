use std::fs;
use regex::Regex;

pub fn problem1(filename: &str) {
    let file_content = fs::read_to_string(filename).expect("Error in reading file");
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut _has_started_moving = false;
    let mut _stacks: Vec<Vec<String>> = Vec::<Vec<String>>::new();
    let mut _has_initialized_stacks = false;

    for line in file_content.lines() {
        if line == "" || &line[0..2] == " 1" {
            continue;
        } else if let Some(captures) = re.captures(line) {
            match (captures.get(1), captures.get(2), captures.get(3)) {
                (Some(qty_str), Some(from_str), Some(to_str)) => {
                    if !_has_started_moving {
                        for stack in _stacks.iter_mut() {
                            stack.reverse();
                        }
                        _has_started_moving = true;
                    }

                    let (qty, from, to): (u32, u32, u32) = (qty_str.as_str().parse().unwrap(), from_str.as_str().parse().unwrap(), to_str.as_str().parse().unwrap());
                    for i in 0..qty {
                        let _from_vec = _stacks.get_mut((from-1) as usize).unwrap();
                        let popped_crate = _from_vec.pop().expect("Expected to have non-empty stack");

                        let _to_vec = _stacks.get_mut((to-1) as usize).unwrap();
                        _to_vec.push(popped_crate);
                    }
                },
                _ => panic!("Unexpected instruction capture")
            }
        } else {
            let crates = extract_create_line(line);

            if !_has_initialized_stacks {
                for i in 0..crates.len() {
                    _stacks.push(Vec::<String>::new());
                }
                _has_initialized_stacks = true;
            }

            for (i, crt) in crates.iter().enumerate() {
                if crt != "" {
                    _stacks[i].push(crt.to_string());
                }
            }
            println!("{:?}", crates);
        }
    }

    let top_of_stacks: Vec<&String> = _stacks.iter().map(|s| s.last().unwrap()).collect();
    println!("Top of stacks: {:?}", top_of_stacks);
}

pub fn problem2(filename: &str) {
    let file_content = fs::read_to_string(filename).expect("Error in reading file");
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut _has_started_moving = false;
    let mut _stacks: Vec<Vec<String>> = Vec::<Vec<String>>::new();
    let mut _has_initialized_stacks = false;

    for line in file_content.lines() {
        if line == "" || &line[0..2] == " 1" {
            continue;
        } else if let Some(captures) = re.captures(line) {
            match (captures.get(1), captures.get(2), captures.get(3)) {
                (Some(qty_str), Some(from_str), Some(to_str)) => {
                    if !_has_started_moving {
                        for stack in _stacks.iter_mut() {
                            stack.reverse();
                        }
                        _has_started_moving = true;
                    }

                    let (qty, from, to): (u32, u32, u32) = (qty_str.as_str().parse().unwrap(), from_str.as_str().parse().unwrap(), to_str.as_str().parse().unwrap());
                    let mut _temp_vec = Vec::<String>::new();
                    for i in 0..qty {
                        let _from_vec = _stacks.get_mut((from-1) as usize).unwrap();
                        let popped_crate = _from_vec.pop().expect("Expected to have non-empty stack");
                        _temp_vec.push(popped_crate);
                        //_to_vec.push(popped_crate);
                    }

                    for i in 0..qty {
                        let _to_vec = _stacks.get_mut((to-1) as usize).unwrap();
                        let popped_crate = _temp_vec.pop().expect("Error expected non empty stack");
                        _to_vec.push(popped_crate);
                    }
                },
                _ => panic!("Unexpected instruction capture")
            }
        } else {
            let crates = extract_create_line(line);

            if !_has_initialized_stacks {
                for i in 0..crates.len() {
                    _stacks.push(Vec::<String>::new());
                }
                _has_initialized_stacks = true;
            }

            for (i, crt) in crates.iter().enumerate() {
                if crt != "" {
                    _stacks[i].push(crt.to_string());
                }
            }
            println!("{:?}", crates);
        }
    }

    let top_of_stacks: Vec<&String> = _stacks.iter().map(|s| s.last().unwrap()).collect();
    println!("Top of stacks: {:?}", top_of_stacks);
}

fn extract_create_line(line: &str) -> Vec<String> {
    let re = Regex::new(r"\[([A-Z])\]").unwrap();
    let mut _crates = Vec::<String>::new();
    let mut _z = line.chars().peekable();
    while _z.peek().is_some() {
        let chunk = _z.by_ref().take(3).collect::<String>();
        let chunk_str = chunk.as_str();
        let capture = re.captures(chunk_str);

        if chunk.eq("   ") {
            _crates.push(String::from(""));
        } else if let Some(cpt) = capture {
            let crate_name_match = cpt.get(1).unwrap();
            let crate_name = String::from(crate_name_match.as_str());
            _crates.push(crate_name);
        } else {

            panic!("Unexpcted chunk");
        }

        if _z.peek().is_some() {
            _z.next();
        }
    }

    return _crates;
}