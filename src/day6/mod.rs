use std::{fs, collections::VecDeque};

pub fn problem2(filename: &str) {
    let file_contents = fs::read_to_string(filename).expect("Error reading file");
    let mut datastream = file_contents.chars();
    let mut _buffer = VecDeque::<char>::with_capacity(3);
    let mut _length = 0;

    // For Problem 1 all I had to do was 0..3
    for _i in 0..13 {
        _length += 1;
        let sig = datastream.next().expect("Expected minimum of 4 chars");
        _buffer.push_back(sig);
    }

    for sig in datastream {
        _length += 1;

        _buffer.push_back(sig);
        if is_all_unique_chars(&_buffer) {
            // Found unique
            println!("Buffer: {:?}", _buffer);
            break;
        }
        _buffer.pop_front();
    }

    println!("Total chars before marker: {} and total length was: {}", _length, file_contents.len());
}

fn is_all_unique_chars(buffer: &VecDeque<char>) -> bool {

    let mut _mask = 0u32;
    for sig in buffer.iter() {
        let sig_int = *sig as u32 - 'a' as u32;
        if _mask & (1 << sig_int) > 0 {
            return false;
        }

        _mask |= 1 << sig_int;
    }

    return true;
}
fn problem1_attempt(filename: &str) {
    let file_contents = fs::read_to_string(filename).expect("Error reading file");
    let mut datastream = file_contents.chars();
    let mut _buffer = VecDeque::<char>::with_capacity(3);

    let mut _mask = 0u32;
    let mut _length = 0u32;
    for _i in 0..3 {
        _length += 1;
        let sig = datastream.next().expect("Expected minimum of 4 chars");
        let sig_int = sig as u32 - 'a' as u32;
        _mask |= 1 << sig_int;
        _buffer.push_back(sig);

    }

    for sig in datastream {
        _length += 1;
        let sig_int = sig as u32 - 'a' as u32;
        if !(_mask & (1 << sig_int) > 0) {
            println!("Buffer: {:?}", _buffer);
            break;
        } else {
            // There was a match hence we need to 
        }

        // Reset start char flag
        let start_char = _buffer.pop_front().expect("Expecting running buffer to always contain at least one item");
        let start_char_int = start_char as u32 - 'a' as u32;
        _mask &= !(1 << start_char_int);

        // Set new flag
        _buffer.push_back(sig);
        _mask |= 1 << sig_int;
    }

    println!("Total chars before marker: {} and total length was: {}", _length, file_contents.len());
}