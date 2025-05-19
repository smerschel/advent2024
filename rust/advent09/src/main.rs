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

    let data = fs::read_to_string(&args[1]).expect("Couldn't open file");

    let mut answer: i64 = part1(&data);
    dev_print!("Data {}", data);
    println!("Part 1 Answer: {}", answer);
    answer = part2(&data) as i64;
    println!("Part 2 Answer: {}", answer);
    return Ok(());
}

fn part1(data: &String) -> i64 {
    let mut filesystem: Vec<i32> = Vec::new();
    let mut is_file: bool = true;
    let mut cur_idx: u32 = 0;
    let mut cur_file: i32 = 0;
    // make filesystem vector
    for c in data.chars() {
        let digit = c.to_digit(10).expect("Got a non-digit character");
        if is_file {
            is_file = false;
            for _ in 0..digit {
                filesystem.push(cur_file);
            }
            cur_file += 1;
        } else {
            is_file = true;
            for _ in 0..digit {
                filesystem.push(-1);
            }
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

fn part2(data: &String) -> u64 {
    let mut is_file: bool = true;
    let mut files: Vec<File> = Vec::new();
    let mut spaces: Vec<Space> = Vec::new(); 
    let mut cur_idx: u32 = 0;
    let mut cur_file: u32 = 0;

    // create files and spaces
    for c in data.chars() {
        let digit = c.to_digit(10).expect("Got a non-digit character");
        if is_file {
            is_file = false;
            files.push(File {id: cur_file, start: cur_idx, size: digit});
            cur_file += 1;
            cur_idx += digit;
        } else {
            is_file = true;
            spaces.push(Space {start: cur_idx, size: digit});
            cur_idx += digit;
        }
    }
    dev_print!("{:?}", files);


    // go through files from back to front, if there is space for the whole file, move it
    let mut fileidx: i32 = files.len() as i32 - 1;
    while fileidx >= 0 {
        let mut spaceidx = 0;
        while spaceidx < spaces.len() {
            if spaces[spaceidx].start > files[fileidx as usize].start {
                break;
            }
            if spaces[spaceidx].size >= files[fileidx as usize].size {
                files[fileidx as usize].start = spaces[spaceidx].start;
                spaces[spaceidx].start += files[fileidx as usize].size;
                spaces[spaceidx].size -= files[fileidx as usize].size;
            }
            spaceidx += 1;
        }
        fileidx -= 1;
    }

    let mut checksum:u64 = 0;
    // calculate checksum
    for file in files.iter() {
        for i in 0..file.size {
            checksum += ((file.start+i) * file.id) as u64;
        }
    }

    return checksum;
}
