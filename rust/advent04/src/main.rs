use std::fs;

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

/// Finds all instances of a string pattern in the grid
///
/// The function searches the entire grid for the first character of the pattern
/// and then checks in all 8 directions (horizontal, vertical, and diagonal) to see if
/// the complete pattern can be formed.
///
/// # Arguments
///
/// * `grid` - A 2D vector of characters to search for patterns
/// * `pattern` - The string pattern to search for
/// * `debug` - Whether to print debug information
///
/// # Returns
///
/// The count of pattern occurrences found in the grid
fn find_string_patterns(grid: &Vec<Vec<char>>, pattern: &str, debug: bool) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;
    
    // Handle empty pattern case
    if pattern.is_empty() {
        return 0;
    }
    
    // Convert pattern to a character vector for easier access
    let pattern_chars: Vec<char> = pattern.chars().collect();
    let first_char = pattern_chars[0];
    let pattern_len = pattern_chars.len();
    
    if debug {
        println!("Searching for pattern: {}", pattern);
        println!("First char: {}", first_char);
        println!("Pattern length: {}", pattern_len);
    }
    
    // Directions: right, down, down-right, down-left, left, up, up-left, up-right
    let directions = [
        (0, 1), (1, 0), (1, 1), (1, -1),
        (0, -1), (-1, 0), (-1, -1), (-1, 1)
    ];
    let direction_names = [
        "right", "down", "down-right", "down-left",
        "left", "up", "up-left", "up-right"
    ];
    
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == first_char {
                if debug {
                    println!("Found first char {} at ({}, {})", first_char, i, j);
                }
                
                // Check each direction
                for (dir_idx, &(di, dj)) in directions.iter().enumerate() {
                    let mut pattern_found = true;
                    
                    // Check if we can fit the entire pattern in this direction
                    for k in 1..pattern_len {
                        let i_k = i as i32 + (di * k as i32);
                        let j_k = j as i32 + (dj * k as i32);
                        
                        // Check bounds
                        if i_k < 0 || i_k >= rows as i32 || j_k < 0 || j_k >= cols as i32 {
                            pattern_found = false;
                            if debug {
                                println!("  Out of bounds at position {} in direction {}", k, direction_names[dir_idx]);
                            }
                            break;
                        }
                        
                        // Check if character matches
                        if grid[i_k as usize][j_k as usize] != pattern_chars[k] {
                            pattern_found = false;
                            if debug {
                                println!("  Mismatch at position {}: expected {}, found {}", 
                                         k, pattern_chars[k], grid[i_k as usize][j_k as usize]);
                            }
                            break;
                        }
                    }
                    
                    if pattern_found {
                        if debug {
                            println!("  Pattern found in direction {}", direction_names[dir_idx]);
                        }
                        count += 1;
                    }
                }
            }
        }
    }
    
    if debug {
        println!("Total patterns found: {}", count);
    }
    
    count
}

/// Finds all 3x3 patterns in the grid where the center is 'A'
///
/// The function searches the entire grid for 'A' characters and checks
/// if they form one of four valid 3x3 patterns:
/// Pattern 1:    Pattern 2:    Pattern 3:    Pattern 4:
/// S . S        S . M        M . S        M . M
/// . A .        . A .        . A .        . A .
/// M . M        S . M        M . S        S . S
///
/// # Arguments
///
/// * `grid` - A 2D vector of characters to search for patterns
/// * `debug` - Whether to print debug information
///
/// # Returns
///
/// The count of 3x3 patterns found in the grid
fn find_3x3_patterns(grid: &Vec<Vec<char>>, debug: bool) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;
    
    // We can't check edges since we need a 3x3 grid
    for i in 1..rows-1 {
        for j in 1..cols-1 {
            if grid[i][j] == 'A' {
                if debug {
                    println!("Found 'A' at ({}, {})", i, j);
                }
                
                // Check all four possible patterns
                let patterns = [
                    // Pattern 1: S.S / .A. / M.M
                    (('S', 'S'), ('M', 'M')),
                    // Pattern 2: S.M / .A. / S.M
                    (('S', 'M'), ('S', 'M')),
                    // Pattern 3: M.S / .A. / M.S
                    (('M', 'S'), ('M', 'S')),
                    // Pattern 4: M.M / .A. / S.S
                    (('M', 'M'), ('S', 'S'))
                ];
                
                for (pattern_idx, &((top_left, top_right), (bottom_left, bottom_right))) in patterns.iter().enumerate() {
                    // Check top row corners
                    if grid[i-1][j-1] == top_left && grid[i-1][j+1] == top_right &&
                       // Check bottom row corners
                       grid[i+1][j-1] == bottom_left && grid[i+1][j+1] == bottom_right {
                        if debug {
                            println!("  Found pattern {} at ({}, {})", pattern_idx + 1, i, j);
                        }
                        count += 1;
                        break; // Found a match, no need to check other patterns
                    }
                }
            }
        }
    }
    
    if debug {
        println!("Total 3x3 patterns found: {}", count);
    }
    
    count
}

/// Creates a 2D grid from a string with newlines
/// 
/// Useful for testing with small grids
/// 
/// # Arguments
/// 
/// * `input` - The input string with newlines representing rows
/// 
/// # Returns
/// 
/// A 2D vector of characters
#[cfg(test)]
fn create_grid_from_string(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| line.chars().collect())
        .collect()
}

/// Main function that reads the input file and counts string patterns
///
/// The program performs the following steps:
/// 1. Read the input file into a 2D grid of characters
/// 2. Search the grid for "XMAS" patterns in all directions
/// 3. Search the grid for special 3x3 patterns with 'A' in center
/// 4. Output both counts
fn main() {
    let grid = read_input_to_2d_array("input.txt");
    
    // Part 1: Find "XMAS" patterns
    let xmas_count = find_string_patterns(&grid, "XMAS", false);
    println!("Part 1: Found {} instances of \"XMAS\"", xmas_count);
    
    // Part 2: Find 3x3 patterns with 'A' in center
    let pattern_count = find_3x3_patterns(&grid, false);
    println!("Part 2: Found {} instances of 3x3 patterns", pattern_count);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Print a grid for debugging
    fn print_grid(grid: &Vec<Vec<char>>) {
        println!("Grid:");
        for row in grid {
            println!("{}", row.iter().collect::<String>());
        }
        println!();
    }
    
    /// Test using a simple grid to verify pattern finding
    #[test]
    fn test_find_string_patterns_simple() {
        // Create a simple test grid with a diagonal XMAS pattern
        // The diagonal goes down-right from X at (0,0)
        let test_grid = create_grid_from_string(
            "XBCDE\n\
             FMCDE\n\
             FFADE\n\
             KLFSE"
        );
        
        print_grid(&test_grid);
        
        // Test finding "XMAS" in diagonal down-right direction
        let count = find_string_patterns(&test_grid, "XMAS", true);
        assert_eq!(count, 1, "Should find 1 instance of XMAS diagonally");
    }
    
    /// Test using a more complex grid with multiple occurrences and directions
    #[test]
    fn test_find_string_patterns_complex() {
        let test_grid = create_grid_from_string(
            "MMMSXXMASM\n\
             MSAMXMSMSA\n\
             AMXSXMAAMM\n\
             MSAMASMSMX\n\
             XMASAMXAMM\n\
             XXAMMXXAMA\n\
             SMSMSASXSS\n\
             SAXAMASAAA\n\
             MAMMMXMMMM\n\
             MXMXAXMASX"
        );
        
        // Test finding "XMAS"
        let count = find_string_patterns(&test_grid, "XMAS", false);
        assert_eq!(count, 18, "Should find 18 XMAS patterns");
        
        // Test finding "SAM"
        let count_sam = find_string_patterns(&test_grid, "SAM", false);
        assert_eq!(count_sam, 38, "Should find 38 SAM patterns");
    }
    
    /// Test empty pattern
    #[test]
    fn test_empty_pattern() {
        let test_grid = create_grid_from_string("ABC\nDEF");
        let count = find_string_patterns(&test_grid, "", false);
        assert_eq!(count, 0, "Empty pattern should return 0");
    }
    
    /// Test pattern longer than grid
    #[test]
    fn test_pattern_too_long() {
        let test_grid = create_grid_from_string("ABC\nDEF");
        let count = find_string_patterns(&test_grid, "ABCDEFGHI", false);
        assert_eq!(count, 0, "Pattern longer than grid should return 0");
    }
    
    /// Test simple directions
    #[test]
    fn test_all_directions() {
        // Simple test case with XMAS in horizontal directions
        let test_grid = create_grid_from_string(
            "....S....\n\
             ....A....\n\
             ....M....\n\
             SAMXMAS..\n\
             ....M....\n\
             ....A....\n\
             ....S...."
        );
        
        print_grid(&test_grid);
        
        // Should find XMAS in right and left directions
        let count = find_string_patterns(&test_grid, "XMAS", true);
        assert_eq!(count, 2, "Should find 2 XMAS patterns (left and right)");
    }
    
    /// Test finding 3x3 patterns
    #[test]
    fn test_find_3x3_patterns() {
        // Test all four patterns
        let test_grid = create_grid_from_string(
            "S.SM.M\n\
             .A..A.\n\
             M.MS.S\n\
             S.MM.S\n\
             .A..A.\n\
             S.MM.S"
        );
        
        print_grid(&test_grid);
        
        // Should find all four patterns
        let count = find_3x3_patterns(&test_grid, true);
        assert_eq!(count, 4, "Should find all four patterns");
        
        // Test with overlapping patterns
        let test_grid2 = create_grid_from_string(
            "S.SS.M\n\
             .A..A.\n\
             M.MS.S\n\
             M.MM.M\n\
             .A..A.\n\
             S.SS.S"
        );
        
        print_grid(&test_grid2);
        
        let count2 = find_3x3_patterns(&test_grid2, true);
        assert_eq!(count2, 3, "Should find 3 patterns (one 'A' doesn't form a valid pattern)");
    }
}
