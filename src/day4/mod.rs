use std::fs;

pub fn problem1(filename: &str) {
    let file_content = fs::read_to_string(filename).expect("Error reading files");

    let mut total_overlaps_count = 0;
    for line in file_content.lines() {
        let split = line.split(",");
        let numbers: Vec<u32> = split
            .flat_map(|s| s.split("-"))
            .map(|n| n.parse::<u32>().expect("Error parsing number"))
            .collect();

        let (l1, l2, r1, r2) = (numbers.get(0), numbers.get(1), numbers.get(2), numbers.get(3));

        let encompases = match (l1, l2, r1, r2) {
            (Some(l1), Some(l2), Some(r1), Some(r2)) => does_encompass(l1, l2, r1, r2),
            _ => panic!("Error getting numbers")
        };

        if encompases {
            total_overlaps_count+=1;
        }
    }

    println!("Total overlaps count: {}", total_overlaps_count);
}

pub fn problem2(filename: &str) {
    let file_content = fs::read_to_string(filename).expect("Error reading files");

    let mut total_overlaps_count = 0;
    for line in file_content.lines() {
        let split = line.split(",");
        let numbers: Vec<u32> = split
            .flat_map(|s| s.split("-"))
            .map(|n| n.parse::<u32>().expect("Error parsing number"))
            .collect();

        let (l1, l2, r1, r2) = (numbers.get(0), numbers.get(1), numbers.get(2), numbers.get(3));

        let encompases = match (l1, l2, r1, r2) {
            (Some(l1), Some(l2), Some(r1), Some(r2)) => does_overlap(l1, l2, r1, r2),
            _ => panic!("Error getting numbers")
        };

        if encompases {
            total_overlaps_count+=1;
        }
    }

    println!("Total overlaps count: {}", total_overlaps_count);

}

fn does_encompass(l1: &u32, l2: &u32, r1: &u32, r2: &u32) -> bool {
    if l1<= r1 && l2 >= r2 {
        return true;
    } else if r1 <= l1 && r2 >= l2 {
        return true;
    }

    return false;
}

fn does_overlap(l1: &u32, l2: &u32, r1: &u32, r2: &u32) -> bool {
    if (r1 >= l1 && r1 <= l2) || (r2 >= l1 && r2 <= l2) {
        return true;
    } else if (l1 >= r1 && l1 <= r2) || (l2 >= r1 && l2 <= r2) {
        return true;
    }

    return false;
}