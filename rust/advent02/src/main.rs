use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?; // Open the file
    let reader = io::BufReader::new(file); // Wrap it in a buffered reader
    let mut safecount: i32 = 0;

    for line in reader.lines() {
        let line = line?; // Unwrap the Result<String>, handling any errors
        //println!("{}", line); // Print each line
        let numbers: Vec<i32> = line.split_whitespace().filter_map(|s| s.parse::<i32>().ok()).collect();
        // brute force it?  I could just try removing one number of every one that fails
        if !is_safe(&numbers) {
            for index in 0..numbers.len() {
                let mut new_vec = numbers.clone();
                new_vec.remove(index);
                if is_safe(&new_vec) {
                    safecount += 1;
                    break;
                }
            }
        }
        else {
            safecount += 1;
        }
    }

    println!("{}", safecount);

    Ok(())
}

fn is_safe(numbers: &Vec<i32>) -> bool {

    // ensure gaps are 1-3
    for window in numbers.windows(2) {
        if (window[0]-window[1]).abs() < 1 || (window[0]-window[1]).abs() > 3 {
            return false;
        }
    }
    // now check all-up or all-down
    let up: bool = numbers.get(0) < numbers.get(1);

    for window in numbers.windows(2) {
        if up {
            if window[1] < window[0] {
                return false;
            }
        } else {
            if window[1] > window[0] {
                return false;
            }
        }
    }
    return true;
}
