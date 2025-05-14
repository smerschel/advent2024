use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

type Point = (usize, usize);

/// Read a file into a 2D vector of characters
fn read_file_to_2d_array(filename: &str) -> Vec<Vec<char>> {
    let file = fs::File::open(filename).expect("Failed to open file");
    let lines = io::BufReader::new(file).lines();
    
    lines.map(|line| {
        line.expect("Failed to read line")
            .chars()
            .collect()
    }).collect()
}

/// Read a file into a string
fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Failed to read file")
}

/// Check if a point is within grid bounds
fn check_limits(anti: &(isize, isize), rows: usize, cols: usize) -> bool {
    anti.0 >= 0 && 
    anti.0 < cols as isize && 
    anti.1 >= 0 && 
    anti.1 < rows as isize
}

/// Generate antinodes based on pairs of nodes
fn generate_antinodes(nodes: &HashMap<char, Vec<Point>>, rows: usize, cols: usize) -> usize {
    let mut antis = Vec::new();
    
    for (_key, points) in nodes {
        for i in 0..points.len() {
            for j in i+1..points.len() {
                let a = points[i];
                let b = points[j];
                
                // Calculate direction vector
                let dx = a.0 as isize - b.0 as isize;
                let dy = a.1 as isize - b.1 as isize;
                
                // Calculate potential antinodes
                let anti1 = (a.0 as isize + dx, a.1 as isize + dy);
                if check_limits(&anti1, rows, cols) {
                    antis.push(anti1);
                }
                
                let anti2 = (b.0 as isize - dx, b.1 as isize - dy);
                if check_limits(&anti2, rows, cols) {
                    antis.push(anti2);
                }
            }
        }
    }
    
    // Remove duplicates by converting to a set
    HashSet::<(isize, isize)>::from_iter(antis.into_iter()).len()
}

/// Get nodes on a line with a given slope and y-intercept
fn get_nodes_on_line(slope: f64, yint: f64, rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut antis = Vec::new();
    let mut x = 0.0;
    let mut y = yint;
    
    while x < cols as f64 {
        // Check if y is close to an integer value
        let y_rounded = y.round();
        if (y - y_rounded).abs() < 1e-10 {
            let y_int = y_rounded as isize;
            
            // Check if y is in range
            if y_int >= 0 && y_int < rows as isize {
                antis.push((x as usize, y_int as usize));
            }
        }
        
        x += 1.0;
        y += slope;
    }
    
    antis
}

/// Generate antinodes with resonance
fn generate_anti_with_resonance(nodes: &HashMap<char, Vec<Point>>, rows: usize, cols: usize) -> usize {
    let mut antis = Vec::new();
    
    for (_key, points) in nodes {
        for i in 0..points.len() {
            for j in i+1..points.len() {
                let a = points[i];
                let b = points[j];
                
                // Avoid division by zero
                if a.0 == b.0 {
                    continue;
                }
                
                // Calculate slope and y-intercept
                let slope = (a.1 as f64 - b.1 as f64) / (a.0 as f64 - b.0 as f64);
                let yint = a.1 as f64 - slope * a.0 as f64;
                
                // Get nodes on this line
                let line_nodes = get_nodes_on_line(slope, yint, rows, cols);
                antis.extend(line_nodes);
            }
        }
    }
    
    // Remove duplicates
    HashSet::<(usize, usize)>::from_iter(antis.into_iter()).len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Get command line arguments
    let filename = if args.len() > 1 { &args[1] } else { "input.txt" };
    let part2 = args.len() > 2 && args[2] == "part2";
    let dev = args.len() > 3 && args[3] == "dev";
    
    // Read data from file
    let data = read_file_to_2d_array(filename);
    let rows = data.len();
    let cols = data[0].len();
    
    // Build nodes map from alphanumeric characters
    let mut nodes: HashMap<char, Vec<Point>> = HashMap::new();
    
    for (i, row) in data.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            if val.is_alphanumeric() {
                nodes.entry(val)
                     .or_insert_with(Vec::new)
                     .push((j, i));
            }
        }
    }
    
    // Part 1: Generate antinodes
    let part1_answer = generate_antinodes(&nodes, rows, cols);
    println!("Part 1: {}", part1_answer);
    
    // Part 2: Generate antinodes with resonance
    let part2_answer = generate_anti_with_resonance(&nodes, rows, cols);
    println!("Part 2: {}", part2_answer);
}