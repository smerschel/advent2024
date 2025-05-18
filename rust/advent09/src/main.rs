use std::fs;
use utils::dev_print;

#[derive(Debug, Clone, Copy)]
struct File {
    id: u32,
    start: u32,
    size: u32,
}

#[derive(Debug, Clone, Copy)]
struct Space {
    start: u32,
    size: u32,
}

/// Main function that reads the input file
///
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // Expect the first argument (after the binary name) to be the filename
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
    }

    let mut data = fs::read_to_string(&args[1]).expect("Couldn't open file");

    let mut answer: i64 = part1(&data);
    dev_print!("Data {}", data);
    println!("Part 1 Answer: {}", answer);
    answer = part2(&data);
    println!("Part 2 Answer: {}", answer);
    return Ok(());
}

fn part1(data: &String) -> i64 {
    let mut filesystem: Vec<i32> = Vec::new();
    let mut isFile: bool = true;
    let mut curIdx: u32 = 0;
    let mut curFile: i32 = 0;
    // make filesystem vector
    for c in data.chars() {
        let digit = c.to_digit(10).expect("Got a non-digit character");
        if isFile {
            isFile = false;
            for _ in 0..digit {
                filesystem.push(curFile);
            }
            curFile += 1;
            curIdx += digit;
        } else {
            isFile = true;
            for _ in 0..digit {
                filesystem.push(-1);
            }
            curIdx += digit;
        }
    }    
    dev_print!("{:?}", filesystem);
    // compress filesystem vector
    let mut i = 0;
    while i < filesystem.len() {
        if filesystem[i] < 0 {
            // Find the last positive value in sub_slice along with its position
            if let Some((pos, _val)) = filesystem[i..].iter()
                                                .enumerate()
                                                .rev()               // reverse iterate
                                                .find(|&(_j, &v)| v > 0) 
            {
                let last_positive_index = i + pos;
                dev_print!("Last positive value at index {}", last_positive_index);

                // Swap with current index element
                filesystem.swap(i, last_positive_index);

                dev_print!("After swap: {:?}", filesystem);
            }
        }
        i += 1;
    }
    // calculate checksum
    let mut checksum: i64 = 0;
    i = 0;
    while i < filesystem.len() {
        if filesystem[i] > 0 {
            checksum += i as i64 * filesystem[i] as i64;
        }
        i += 1;
    }

    return checksum;
}

fn part2(data: &String) -> i64 {
    let mut isFile: bool = true;
    let mut files: Vec<File> = Vec::new();
    let mut spaces: Vec<Space> = Vec::new(); 
    let mut curIdx: u32 = 0;
    let mut curFile: u32 = 0;

    for c in data.chars() {
        let digit = c.to_digit(10).expect("Got a non-digit character");
        if isFile {
            isFile = false;
            files.push(File {id: curFile, start: curIdx, size: digit});
            curFile += 1;
            curIdx += digit;
        } else {
            isFile = true;
            spaces.push(Space {start: curIdx, size: digit});
            curFile += 1;
            curIdx += digit;
        }
    }
    return 0;
}