use std::fs;

pub fn problem1(filename: &str) {
    let file_contents = fs::read_to_string(filename).expect("Error reading file");

    let mut _register = 1;
    let mut _cycle = 0;
    let mut _signal = 0;
    let mut _next_cycle_to_calculate_signal = 20;
    let next_cycles_to_add  = 40;

    let mut run_cycle = |register_val: &i32| {
        _cycle += 1;
        if _cycle == _next_cycle_to_calculate_signal {
            _signal += _cycle * *register_val;
            _next_cycle_to_calculate_signal += next_cycles_to_add;
        }
    };

    for line in file_contents.lines() {
        match line {
            "noop" => run_cycle(&_register),
            operation => {
                let num = String::from_iter(operation.chars().skip(5)).parse::<i32>().unwrap();
                for _ in 0..2 {
                    run_cycle(&_register);
                }
                _register += num;
            }
        }
    }

    println!("Signal value: {}", _signal);
}

pub fn problem2(filename: &str) {
    let file_contents = fs::read_to_string(filename).expect("Error reading file");

    let mut _register = 1;
    let mut _cycle = 0;
    let mut _signal = 0;
    let mut _pixel_position = 0;
    let next_cycles_to_add  = 40;

    let mut run_cycle = |register_val: &i32| {
        _cycle += 1;

        if _pixel_position >= *register_val-1 && _pixel_position <= *register_val+1 {
            print!("#");
        } else {
            print!(".")
        }

        _pixel_position = (_pixel_position + 1) % next_cycles_to_add;
        if _pixel_position == 0 {
            println!();
        }
    };

    for line in file_contents.lines() {
        match line {
            "noop" => run_cycle(&_register),
            operation => {
                let num = String::from_iter(operation.chars().skip(5)).parse::<i32>().unwrap();
                for _ in 0..2 {
                    run_cycle(&_register);
                }
                _register += num;
            }
        }
    }

    println!("Signal value: {}", _signal);
}