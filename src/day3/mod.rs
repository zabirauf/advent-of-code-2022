use std::fs;

pub fn problem1(filename: &str) {
    let file_contents = fs::read_to_string(filename).expect("Error reading file");

    let mut _priority_sum = 0;
    for line in file_contents.lines() {
        let half_size = line.len() / 2;
        let l_str= line.chars().take(half_size).collect::<String>();
        let r_str= line.chars().skip(half_size).take(half_size).collect::<String>();

        let duplicate = find_duplicate_char(&l_str, &r_str);


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
                //println!("ARR: {:?}, D: {}, PS: {}", _elf_group, dup, _priority_sum);
            } else {
                panic!("Error: No duplicate found");
            }

            _current_elf = 0;
        }
    }
 
    println!("Total priority: {}", _priority_sum);
}

fn find_duplicate_char(l_str: &str, r_str: &str) -> Option<char> {
    let mut _l_iter = 0;
    let mut _r_iter = 0;
    let mut _sorted_lstr: Vec<char> = l_str.chars().collect::<Vec<_>>();
    let mut _sorted_rstr = r_str.chars().collect::<Vec<_>>();

    _sorted_lstr.sort();
    _sorted_rstr.sort();

    let str_len = l_str.len();

    while _l_iter < str_len && _r_iter < str_len {
        let l_char = _sorted_lstr[_l_iter];
        let r_char = _sorted_rstr[_r_iter];
        
        if l_char == r_char {
            return Some(l_char);
        } else if l_char < r_char {
            _l_iter += 1;
        } else {
            _r_iter += 1;
        }
    }

    return None;
}

fn find_duplicate_char_in_group(strs: [&str; 3]) -> Option<char> {
    let mut c_strs = strs.map(|s| s.chars().collect::<Vec<_>>());
    for cs in c_strs .iter_mut(){
        cs.sort();
    }

    let total_indexes = 3;
    let mut indexes: [usize; 3] = [0, 0, 0];

    // Infinite loop as the problem dictates that there will be at least one matching across
    loop {
        let mut is_all_same = true;
        for i in 1..total_indexes {
            let curr_index: usize = indexes[i].try_into().unwrap();
            let prev_index: usize = indexes[i-1].try_into().unwrap();

            // println!("LSize: {}:{}[{}], RSize: {}:{}[{}]", i-1, strs[i-1].len(), prevIndex, i, strs[i].len(), currIndex);
            let curr_char = c_strs[i].get(curr_index);
            let prev_char = c_strs[i-1].get(prev_index);

            is_all_same = is_all_same && prev_char == curr_char;

            if !is_all_same {
                break;
            }
        }

        if is_all_same {
            let first_index: usize = indexes[0].try_into().unwrap();
            let curr_char = c_strs[0].get(first_index).unwrap();
            return Some(curr_char.clone());
        }

        let min_char_val = indexes.iter().enumerate()
            .map(|(i, index)| { return (i, c_strs[i].get(index.clone())) })
            .min_by(|(_, lindex), (_, rindex)| lindex.cmp(rindex));

        match min_char_val {
            Some((i, _)) => indexes[i] += 1,
            None => panic!("Unexpected none")
        }

    }
}

fn get_priority(c: char) -> u32 {
    let c_int = c as u32;
    //print!("PRI: {}, 'a': {}, 'A': {} ;", cInt, 'a' as u32, 'A' as u32);

    if c_int > ('Z' as u32) {
        return c_int - ('a' as u32) + 1;
    }

    return c_int - ('A' as u32) + 27;
}