use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let mut count = -1;
    let mut prevDepth = -1;

    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(depth) = line {
                let temp = depth.parse::<i32>().unwrap();
                if prevDepth < temp {
                    count += 1;
                }
                prevDepth = temp;
                // count += 1;
            }
        }
    }

    println!("Number of lines: {}", count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
