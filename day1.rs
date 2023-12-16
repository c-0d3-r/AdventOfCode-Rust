use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum: i32 = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(s) = line {
                let nums: String = s.chars().filter(|c| c.is_digit(10)).collect();

                if let (Some(first), Some(last)) = (nums.chars().next(), nums.chars().rev().next())
                {
                    let combined = format!("{}{}", first, last);

                    if let Ok(result) = combined.parse::<i32>() {
                        sum += result;
                    }
                }
            }
        }
    }

    println!("{sum}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
