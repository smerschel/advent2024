use std::fs;
use utils::dev_print;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Main function that reads the input file
///
fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Expect the first argument (after the binary name) to be the filename
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
    }
    
    let grid = read_input_to_2d_array(&args[1]);
    
    let (startrow, startcol, startdir) = find_start(&grid);

    // while not at_exit, mark current position with an "X" and move
    let mut row = startrow;
    let mut col = startcol;
    let mut dir = startdir;
    let mut map = grid.clone();

    loop {
        // mark current position with an "X"
        map[row][col] = 'X';

        // check for exit
        if at_exit(row, col, map.len(), map[0].len(), dir) {
            break;
        }

        // move in the current direction
        do_move(&mut row, &mut col, &mut dir, &mut map);
    }

    // print the number of x's
    dev_print!("Number of X's: {}", count_xs(&map));
    print!("Number of X's: {}", count_xs(&map));
    println!();
    
}


/// Reads an input file and converts it to a 2D array of characters
///
/// # Arguments
///
/// * `filename` - Path to the input file
///
/// # Returns
///
/// A 2D vector of characters representing the grid from the input file
fn read_input_to_2d_array(filename: &str) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(filename)
        .expect("Failed to read input file");
    
    let grid: Vec<Vec<char>> = contents.lines()
        .map(|line| line.chars().collect())
        .collect();
    
    println!("Grid dimensions: {}x{}", grid.len(), grid[0].len());
    
    grid
}

fn find_start(map: &Vec<Vec<char>>) -> (usize, usize, Direction) {
    for (row, line) in map.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c == '^' {
                return (row as usize, col as usize, Direction::Up);
            } else if *c == '>' {
                return (row as usize, col as usize, Direction::Right);
            } else if *c == 'v' {
                return (row as usize, col as usize, Direction::Down);
            } else if *c == '<' {
                return (row as usize, col as usize, Direction::Left);
            }
        }
    }
    // if we don't find a start, return 0,0 and up
    (0, 0, Direction::Up)
}

fn at_exit(row: usize, col: usize, rows: usize, cols: usize, dir: Direction) -> bool {
    // check if we are at the edge of the map
    match dir {
        Direction::Up => row == 0,
        Direction::Right => col == cols - 1,
        Direction::Down => row == rows - 1,
        Direction::Left => col == 0,
    }
}

fn turn_right(dir: &mut Direction) {
    *dir = match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    };
}

fn do_move(row: &mut usize, col: &mut usize, dir: &mut Direction, map: &mut Vec<Vec<char>>) {
    // print column and row
    dev_print!("Current position: ({}, {})", col, row);
    // check for # in front of current position
    match dir {
        Direction::Up => {
            // print map[*col][*row - 1]
            dev_print!("Map: {} ", map[*col][*row - 1]);
            dev_print!("Map: {} ", map[*row-1][*col]);
            if map[*row-1][*col] == '#' {
                turn_right(dir);
            } else {
                *row -= 1;
            }
        }
        Direction::Right => {
            if map[*row][*col+1] == '#' {
                turn_right(dir);
            } else {
                *col += 1;
            }
        }
        Direction::Down => {
            if map[*row+1][*col] == '#' {
                turn_right(dir);
            } else {
                *row += 1;
            }
        }
        Direction::Left => {
            if map[*row][*col-1] == '#' {
                turn_right(dir);
            } else {
                *col -= 1;
            }
        }
    }
}

fn count_xs(map: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for line in map.iter() {
        for c in line.iter() {
            if *c == 'X' {
                count += 1;
            }
        }
    }
    count
}