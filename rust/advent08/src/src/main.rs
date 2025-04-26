use std::fs::File;
use std::io::{self, BufRead};
use utils::dev_print;

/// Main function that reads the input file
///
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // Expect the first argument (after the binary name) to be the filename
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
    }
    
    let file = File::open(&args[1])?; // Open the file
    let reader = io::BufReader::new(file); // Wrap it in a buffered reader
    let mut answer: i64 = 0;

    for line in reader.lines() {
        let line = line?; // Unwrap the Result<String>, handling any errors
        dev_print!("{}", line); // Print each line
        // split line on colon delimiter
        if let Some((resultstr, vars_str)) = line.split_once(':') {
            // `result` and `vars` are both &str
            println!("Result: {}", resultstr);
            println!("Vars: {}", vars_str);
            // convert resultstr and vars_str to i64's
            let result: i64 = resultstr.parse::<i64>().unwrap();
            // Split the string by spaces, parse each part as i64, and collect into a Vec<i64>
            let vars: Vec<i64> = vars_str
                .split_whitespace()     // Split the string by whitespace
                .map(|s| s.parse().unwrap()) // Parse each part as an i64, or unwrap if it fails
                .collect();            // Collect into a vector
            

            answer += compute(result, vars);
        }                // split part
    }
    print!("Answer: {}", answer);
    return Ok(());

}

fn compute(result: i64, vars: Vec<i64>) -> i64 {
    let mut results: Vec<i64> = Vec::new();
    results.push(vars[0]);
    // for remaining vars, call calculate, which will modify the results parameters
    for var in vars.into_iter().skip(1) {
        calculate(var, &mut results);
    }
    // check if result is in results
    if results.contains(&result) {
        return result;
    }
    return 0;
}

fn calculate(var: i64, results: &mut Vec<i64>) {
    let mut newresults: Vec<i64> = Vec::new();
    // for each result, calculate the result of var + result
    for result in &mut *results {
        newresults.push(var + *result);
        newresults.push(var * *result);
        // if part2 uncomment this, concatenate result and var strings
        let resstr = format!("{}{}", result, var);
        newresults.push(resstr.parse::<i64>().unwrap())   
    }
    // replace results with newresults
    *results = newresults;
}
