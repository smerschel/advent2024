use std::fs::File;
use std::io::{self, BufRead};
use utils::dev_print;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // Expect the first argument (after the binary name) to be the filename
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
    }

    let file = File::open(&args[1])?; // Open the file
    let reader = io::BufReader::new(file); // Wrap it in a buffered reader

    // declare a vector of integer pairs
    let mut rules: Vec<(i32, i32)> = Vec::new();
    // declare a vector of vectors of integers
    let mut updates: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line?; // Unwrap the Result<String>, handling any errors
        dev_print!("{}", line); // Print each line
        // if a line contains a pipe, split it into two parts
        if line.contains('|') {
            let parts: Vec<&str> = line.split('|').collect();
            // parse the first part as a tuple of integers
            let rule: (i32, i32) = (
                parts[0].trim().parse().unwrap(),
                parts[1].trim().parse().unwrap(),
            );
            rules.push(rule);
        } else {
            // if it contains a comma, delimited list of integers parse it and put it in the updates vector
            let parts: Vec<i32> = line
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            // only push it onto updates if it's not empty
            if !parts.is_empty() {
                updates.push(parts);
            }
        }
    }
    // print rules and updatesupdate.iter().position(|&x| x == b)
    dev_print!("Rules: {:?}", rules);
    dev_print!("Updates: {:?}", updates);

    // iterate over the rules and make a hashmap with the first integer as the key mapping to a vector of the second integers
    let mut before_map: std::collections::HashMap<i32, Vec<i32>> = std::collections::HashMap::new();
    for rule in &rules {
        before_map
            .entry(rule.0)
            .or_insert_with(Vec::new)
            .push(rule.1);
    }

    // print the map
    dev_print!("Before Map: {:?}", before_map);

    // initialize a counter for good updates and a vector to store middle values of good updates
    let mut good = 0;
    let mut good_updates: Vec<i32> = Vec::new();
    // create a vector to store bad updates
    let mut bad_updates: Vec<Vec<i32>> = Vec::new();

    // iterate over the updates
    for update in &updates {
        if check_good(update, &before_map).0 {
            // if the update is good, increment the counter
            if !update.is_empty() {
                let middle_index = update.len() / 2;
                good_updates.push(update[middle_index]);
                good += 1;
            }
        } else {
            bad_updates.push(update.clone());
        }
    }

    if args.len() > 2 && args[2] == "part2" {
        fix_and_count_bad(bad_updates.clone(), &before_map);
    }

    println!("Total good updates: {}", good);
    dev_print!("Good updates middle values: {:?}", good_updates);
    dev_print!("Bad updates: {:?}", bad_updates);

    // sum up the good middle values and print the sum
    let sum: i32 = good_updates.iter().sum();
    println!("Sum of good updates: {}", sum);

        
    Ok(())
}

fn fix_and_count_bad(bad_updates: Vec<Vec<i32>>, before_map: &std::collections::HashMap<i32, Vec<i32>>) {
    let mut sum: i32 = 0;
    // Iterate over the bad updates
    for mut update in bad_updates {
        let mut result: (bool, (i32,i32));
        dev_print!("********New Update**********");
        while { result = check_good(&update, before_map);
                !result.0
        } {
            // print update
            dev_print!("Bad update: {:?}", update);
            dev_print!("result: {:?}", result);
            // if the bad is in the before_map, that means we need to move this number earlier
            update.swap(result.1.0.try_into().unwrap(), result.1.1.try_into().unwrap());
            // print update
            dev_print!("Shifted update: {:?}", update);
        }
        // now the update is good, count the middle value of the corrected update and store it in a count variable
        let middle_index = update.len() / 2;
        let middle_value = update[middle_index];
        // print the middle value
        dev_print!("Middle value of corrected update: {}", middle_value);
        // print the corrected update
        dev_print!("Corrected update: {:?}", update);
        sum += middle_value;
    }
    // print the sum of the middle values
    println!("Sum of middle values of corrected updates: {}", sum);

}

// if it's bad, it returns false and the indices of the two numbers that were violated a rule
fn check_good(update: &Vec<i32>, before_map: &std::collections::HashMap<i32, Vec<i32>>) -> (bool, (i32, i32)) {
    // Check if the update is good

    // iterate over each integer in the update
    for (i, &num) in update.iter().enumerate() {
        // create two vectors, one should be the items in the update before the current number and one should be the items after the current number
        let before: Vec<i32> = update[..i].to_vec();
        // now lookup the hashmaps for this number from rules
        // and get the before and after vectors
        let empty_vec: Vec<i32> = Vec::new();  // Create a long-lived, empty vector
        let before_vec = before_map.get(&num).unwrap_or(&empty_vec);
        // if any numbers are in both before and before_vec, this is a bad update
        // if any numbers are in both after and after_vec, this is a bad update
        // set a bad flag in those conditions and goto the next update
        for &b in before_vec {
            if before.contains(&b) {
                // print before and before_vec
                dev_print!("Update: {:?}", update);
                dev_print!("num: {}", num);
                dev_print!("Before: {:?}", before);
                dev_print!("Before_vec: {:?}", before_vec);
                dev_print!("Bad update: {} in both before and before_vec", b);
                return (false, (i.try_into().unwrap(), update.iter().position(|&x| x == b).unwrap().try_into().unwrap())); // exit the loop for this update
            }
        }
    }

    // if we reach here, the update is good
    dev_print!("Good update: {:?}", update);

    return (true, (0,0)); // return true for a good update

}