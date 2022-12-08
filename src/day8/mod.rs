use std::fs;

pub fn problem1(filename: &str) {
    let file_contents = fs::read_to_string(filename).expect("Error reading filename");

    let mut grid = Vec::<Vec<u8>>::new();

    for line in file_contents.lines() {
        let all_numbers: Vec<u8> = line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
        grid.push(all_numbers);
    }

    let total_rows = grid.len();
    let total_cols = grid.get(0).unwrap().len();

    let mut total_visible = 2*total_cols + 2*total_rows - 4;
    //println!("Before starting: {}", total_visible);
    for row in 1..total_rows-1 {
        for col in 1..total_cols-1 {
            let adjacent_trees = [Direction::Up, Direction::Down, Direction::Left, Direction::Right].map(|direction| is_hidden_in_direction(&grid, total_rows, total_cols, row, col, direction));
            if adjacent_trees.iter().any(|hidden| *hidden == false) {
                // let current = grid[row][col];
                // println!("Visible grid[{}][{}] = {} -> {:?}", row, col, current, adjacent_trees);
                total_visible += 1;
            }
        }
    }

    println!("Total visible tree: {}", total_visible);
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn is_hidden_in_direction(grid: &Vec<Vec<u8>>, total_rows: usize, total_cols: usize, row: usize, col: usize, direction: Direction) -> bool {
    let current = grid[row][col];
    return match direction {
        Direction::Up => (0..row).map(|r| grid[r][col]).any(|n| n >= current),
        Direction::Down => (row+1..total_rows).map(|r| grid[r][col]).any(|n| n >= current),
        Direction::Left => (0..col).map(|c| grid[row][c]).any(|n| n >= current),
        Direction::Right => (col+1..total_cols).map(|c| grid[row][c]).any(|n| n >= current)
    }
}