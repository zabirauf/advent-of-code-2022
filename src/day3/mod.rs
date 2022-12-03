use std::fs;

pub fn problem1(filename: &str) {
    let file_contents = fs::read_to_string(filename).expect("Error reading file");

    let mut _priority_sum = 0;
    for line in file_contents.lines() {
        let halfSize = line.len() / 2;
        let lStr= line.chars().take(halfSize).collect::<String>();
        let rStr= line.chars().skip(halfSize).take(halfSize).collect::<String>();

        let duplicate = find_duplicate_char(&lStr, &rStr);


        if let Some(dup) = duplicate {
            _priority_sum += get_priority(dup);
            //println!("O: {}, L: {}, R: {}, D: {}, PS: {}", line, lStr, rStr, dup, _priority_sum);
        } else {
            panic!("Error: No duplicate found");
        }
    }

    println!("Total priority: {}", _priority_sum);
}

pub fn problem2(filename: &str) {
    let file_contents = fs::read_to_string(filename).expect("Error reading file");

    let mut _priority_sum = 0;
    let mut _elf_group = ["", "", ""];
    let mut _current_elf = 0;
    for line in file_contents.lines() {
        _elf_group[_current_elf] = line;
        _current_elf += 1;

        if _current_elf == 3 {
            let duplicate = find_duplicate_char_in_group(_elf_group);

            if let Some(dup) = duplicate {
                _priority_sum += get_priority(dup);
                println!("ARR: {:?}, D: {}, PS: {}", _elf_group, dup, _priority_sum);
            } else {
                panic!("Error: No duplicate found");
            }

            _current_elf = 0;
        }
    }
 
    println!("Total priority: {}", _priority_sum);
}

fn find_duplicate_char(lStr: &str, rStr: &str) -> Option<char> {
    let mut _lIter = 0;
    let mut _rIter = 0;
    let mut _sorted_LStr: Vec<char> = lStr.chars().collect::<Vec<_>>();
    let mut _sorted_RStr = rStr.chars().collect::<Vec<_>>();

    _sorted_LStr.sort();
    _sorted_RStr.sort();

    let strLen = lStr.len();

    while _lIter < strLen && _rIter < strLen {
        let lChar = _sorted_LStr[_lIter];
        let rChar = _sorted_RStr[_rIter];
        
        if lChar == rChar {
            return Some(lChar);
        } else if lChar < rChar {
            _lIter += 1;
        } else {
            _rIter += 1;
        }
    }

    return None;
}

fn find_duplicate_char_in_group(strs: [&str; 3]) -> Option<char> {
    let lengths: Vec<usize> = strs.iter().map(|s| s.len()).collect();
    let minLen = lengths.iter().min().unwrap();
    let mut cStrs = strs.map(|s| s.chars().collect::<Vec<_>>());
    for cs in cStrs .iter_mut(){
        cs.sort();
    }

    let totalIndexes = 3;
    let mut indexes: [usize; 3] = [0, 0, 0];

    // Infinite loop as the problem dictates that there will be at least one matching across
    while indexes.iter().max().unwrap() < minLen {
        for i in 1..totalIndexes {
            let currIndex: usize = indexes[i].try_into().unwrap();
            let prevIndex: usize = indexes[i-1].try_into().unwrap();

            // println!("LSize: {}:{}[{}], RSize: {}:{}[{}]", i-1, strs[i-1].len(), prevIndex, i, strs[i].len(), currIndex);
            let currChar = cStrs[i].get(currIndex);
            let prevChar = cStrs[i-1].get(prevIndex);

            match (currChar, prevChar) {
                (Some(cc), Some(pc)) => {
                    if cc == pc && i == (totalIndexes - 1) {
                        return Some(cc.clone());
                    } else if cc == pc {
                        continue;
                    } else if pc < cc {
                        indexes[i-1]+= 1;
                    } else if pc > cc {
                        indexes[i] += 1;
                    }
                },

                _ => panic!("Reached end: {}, {}", strs[i-1], strs[i])
            }
        }
    }

    return None;
}

fn get_priority(c: char) -> u32 {
    let cInt = c as u32;
    //print!("PRI: {}, 'a': {}, 'A': {} ;", cInt, 'a' as u32, 'A' as u32);

    if cInt > ('Z' as u32) {
        return cInt - ('a' as u32) + 1;
    }

    return cInt - ('A' as u32) + 27;
}