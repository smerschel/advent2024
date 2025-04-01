use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "input.txt"; // Change to your file name
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    // Open the file
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // Read each line and parse the integers
    for line in reader.lines() {
        let line = line?; // Unwrap the Result<String>
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.len() == 2 {
            if let (Ok(num1), Ok(num2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                list1.push(num1);
                list2.push(num2);
            } else {
                eprintln!("Skipping invalid line: {}", line);
            }
        } else {
            eprintln!("Skipping malformed line: {}", line);
        }
    }
    //list1 = Vec::new();
    //list2 = Vec::new();
    //list1.extend(vec![3,4,2,1,3,3]);
    //list2.extend(vec![4,3,5,3,9,3]);

    let mut sorted_vec1 = list1.clone();
    let mut sorted_vec2 = list2.clone();

    sorted_vec1.sort();
    sorted_vec2.sort();

    // Print results
    println!("List 1: {:?}", sorted_vec1);
    println!("List 2: {:?}", sorted_vec2);

    let mut distance: i32 = 0;

    // computer differences
    if sorted_vec1.len() != sorted_vec2.len() {
        eprintln!("Lists are different sizes: {}, {}", sorted_vec1.len(), sorted_vec2.len());
    } else {
        for (a,b) in sorted_vec1.iter().zip(sorted_vec2.iter()) {
            println!("a: {} b:{} dist:{}", a, b, (a-b).abs());
            distance += (a-b).abs();
        }
    }

    println!("Distance: {:?}", distance);

    // now do similarity
    let mut similarity: i32 = 0;
    let mut count: i32 = 0;
    for a in list1.iter() {
        count = 0;
        for b in list2.iter() {
            if (a==b) {
                count += 1;
            }
        }
        similarity += a*count;
    }

    println!("Similarity: {:?}", similarity);

    Ok(())
}