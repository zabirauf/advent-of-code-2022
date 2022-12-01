use std::fs;
use std::cmp;
pub fn problem1(filename: &str) {
    let file_contents = fs::read_to_string(filename).expect("Error reading file");

    let mut _current_calorie_count = 0;
    let mut _max_calorie_count = i32::MIN;
    for line in file_contents.lines() {
        match line {
            "" => {
                _max_calorie_count = cmp::max(_max_calorie_count, _current_calorie_count);
                _current_calorie_count = 0;
            }
            _ => {
                _current_calorie_count += line.parse::<i32>().expect("Error, expecting number");
            }
        }
    }

    println!("Max calorie count is : {}", _max_calorie_count);
}

pub fn problem2(filename: &str) {
    let file_contents = fs::read_to_string(filename).expect("Error reading file");

    let mut _current_calorie_count = 0;
    let mut _elf_calorie_count = Vec::<i32>::new();
    for line in file_contents.lines() {
        match line {
            "" => {
                _elf_calorie_count.push(_current_calorie_count);
                _current_calorie_count = 0;
            }
            _ => {
                _current_calorie_count += line.parse::<i32>().expect("Error, expecting number");
            }
        }
    }

    _elf_calorie_count.sort_by(|a, b| b.cmp(a));
    // let sum: &i32 = &_elf_calorie_count[0..=2].iter().sum();
    let sum: i32 = _elf_calorie_count.iter().take(3).sum();

    println!("Max calorie count is : {:?}", sum);
}