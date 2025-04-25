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

    // part two
    // reset variables
    let (startrow, startcol, startdir) = find_start(&grid);
    let mut row = startrow;
    let mut col = startcol;
    let mut dir = startdir;
    let mut map2 = grid.clone();

    // make an empty vector to track row and columns of added obstacles
    let mut obstacles: Vec<(usize, usize)> = Vec::new();
    loop {
        // if not next to exit (we can't add an obstacle at the edge of the map)
        if !at_exit(row, col, map2.len(), map2[0].len(), dir) {
            // add "O" in front of current position
            dev_print!("Adding obstacle at ({}, {})", col, row);
            let (obsrow, obscol) = add_obstacle(&mut row, &mut col, &mut dir, &mut map2);
            // run sim
            let mut simrow = startrow;
            let mut simcol = startcol;
            let mut simdir = startdir;
            if run_simulation(&mut simrow, &mut simcol, &mut simdir, &mut map2) {
                // append obsrow and obscol to obstacles, if it doesn't already exist
                if !obstacles.contains(&(obsrow, obscol)) {
                    obstacles.push((obsrow, obscol));
                }
            }
            // remove "O" in front of current position
            dev_print!("Removing obstacle at ({}, {})", col, row);
            remove_obstacle(&mut row, &mut col, &mut dir, &mut map2);
        } else {
            break;
        }

        // move in the current direction
        do_move(&mut row, &mut col, &mut dir, &mut map2);
    }
    // print the number of obstacles
    print!("Number of obstacles: {}", obstacles.len());

}

fn add_obstacle(row: &mut usize, col: &mut usize, dir: &mut Direction, map: &mut Vec<Vec<char>>) -> (usize, usize) {
    // don't add one if there is already one there
    if !check_obstacle(row, col, dir, map, '#') {
        if !check_obstacle(row, col, dir, map, '^') {
            // add "O" in front of current position
            match dir {
                Direction::Up => { 
                    map[*row - 1][*col] = 'O';
                    return (*row - 1, *col)
                },
                Direction::Right => { 
                    map[*row][*col + 1] = 'O';
                    return (*row, *col + 1)
                },
                Direction::Down => { 
                    map[*row + 1][*col] = 'O';
                    return (*row + 1, *col)
                },
                Direction::Left => { 
                    map[*row][*col - 1] = 'O';
                    return (*row, *col - 1)
                },
            }
        }
    }
    return (0,0);
}

fn remove_obstacle(row: &mut usize, col: &mut usize, dir: &mut Direction, map: &mut Vec<Vec<char>>) {
    // don't remove it if we didn't add it
    if check_obstacle(row, col, dir, map, 'O') {
        // remove "O" in front of current position
        match dir {
            Direction::Up => map[*row - 1][*col] = '.',
            Direction::Right => map[*row][*col + 1] = '.',
            Direction::Down => map[*row + 1][*col] = '.',
            Direction::Left => map[*row][*col - 1] = '.',
        }
    }
}

fn run_simulation(row: &mut usize, col: &mut usize, dir: &mut Direction, map: &mut Vec<Vec<char>>) -> bool {
    let mut step_count = 0;
    // make a copy of the map
    let mut simmap = map.clone();
    // run the simulation until we hit an obstacle
    loop {
        // check for exit
        if at_exit(*row, *col, simmap.len(), simmap[0].len(), *dir) {
            return false;
        }

        // move in the current direction
        do_move(row, col, dir, &mut simmap);

        step_count += 1;

        if step_count > (simmap.len() * simmap[0].len()) {
                // if we have moved more than the number of cells in the map, we are in an infinite loop
            return true;
        }
    }
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
    if check_obstacle(row, col, dir, map, '#') {
        turn_right(dir);
    } else if check_obstacle(row, col, dir, map, 'O') {
        // if we are next to an obstacle, turn right
        turn_right(dir);
    } else {
        move_forward(row, col, dir);
    }
}

fn check_obstacle(row: &usize, col: &usize, dir: &Direction, map: &Vec<Vec<char>>, symbol: char) -> bool {
    // check for # in front of current position
    match dir {
        Direction::Up => map[*row - 1][*col] == symbol,
        Direction::Right => map[*row][*col + 1] == symbol,
        Direction::Down => map[*row + 1][*col] == symbol,
        Direction::Left => map[*row][*col - 1] == symbol,
    }
}

fn move_forward(row: &mut usize, col: &mut usize, dir: &mut Direction) {
    // move in the current direction
    match dir {
        Direction::Up => *row -= 1,
        Direction::Right => *col += 1,
        Direction::Down => *row += 1,
        Direction::Left => *col -= 1,
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