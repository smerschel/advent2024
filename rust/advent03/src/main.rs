use regex::Regex;
use std::fs;

#[macro_export]
macro_rules! dev_println {
    ($( $args:expr ),*) => {
        #[cfg(feature = "dev_print")]
        {
            println!($( $args ),*);
        }
    };
}

fn parse_with_delimiters(input: &str, on: &str, off: &str) -> String {
    let mut result = String::new();
    let mut in_on_state = true; // Track whether we're in the "on" state
    let mut i = 0;

    while i < input.len() {
        // Find the next "on" or "off" delimiter
        if input[i..].starts_with(on) {
            // If we're in "off" state, now turn "on"
            in_on_state = true;
            i += on.len(); // Move past the "on" delimiter
        } else if input[i..].starts_with(off) {
            // If we're in "on" state, now turn "off"
            in_on_state = false;
            i += off.len(); // Move past the "off" delimiter
        } else {
            // If we're in the "on" state, collect characters
            if in_on_state {
                result.push(input.chars().nth(i).unwrap());
            }
            i += 1; // Move to the next character
        }
    }

    result
}

//fn main() {
//    let input = "some text do() this part should be included don't() but this part should be ignored do() and this part too don't()";
//    let on = "do()";
//    let off = "don't()";
    
//    let result = parse_with_delimiters(input, on, off);
//    println!("Result: {}", result); // Output: "this part should be included and this part too"
//}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let text = fs::read_to_string("input.txt")?;
    let mut my_str = text;

    // check if we are running part 1 or part 2, default to part 1
    if args.len() > 1 && args[1] == "part2" {
        let on = "do()";
        let off = "don't()";
        my_str = parse_with_delimiters(&my_str, on, off);
    } 

    // Define a regex pattern to match numbers
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();  

    let mut total: i32 = 0;
    // Iterate over the matches
    for mat in re.find_iter(&my_str) {
        // Get the matched substring
        let matched_str = mat.as_str();

        // Use captures to extract the two integers
        if let Some(captures) = re.captures(matched_str) {
            let num1: i32 = captures[1].parse().unwrap();
            let num2: i32 = captures[2].parse().unwrap();

            // Convert captures[1] and captures[2] to String for printing
            let _num1_str = captures[1].to_string();
            let _num2_str = captures[2].to_string();

            // Print the result
            dev_println!("Found match: mul({}, {}), num1 = {}, num2 = {}", 
                     _num1_str, _num2_str, num1, num2);
            total += num1*num2;

        }
    }
    println!("Total {}", total);
    Ok(())
}
