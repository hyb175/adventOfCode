use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> Result<Vec<String>, io::Error>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    io::BufReader::new(file).lines().collect::<Result<Vec<String>, io::Error>>()
}

fn part1(lines: Vec<String>) {
    let mut last = -1;
    let mut count = 0;

    for line in lines {
        if let Ok(depth) = line.parse::<i32>() {
            if last > -1 && depth > last {
                count += 1;
            }
            last = depth;
        }
    }

    println!("Part 1: {}", count);
}

fn part2(lines: Vec<String>) {
    let mut last_sum = -1;
    let mut count = 0;
    let mut sliding_window: Vec<i32> = Vec::new();

    for line in lines {
        if let Ok(depth) = line.parse::<i32>() {
            sliding_window.push(depth);
            if sliding_window.len() > 3 {
                sliding_window.remove(0);
            }

            if sliding_window.len() == 3 && sliding_window.iter().sum::<i32>() > last_sum && last_sum > -1 {
                count += 1;
            }

            if sliding_window.len() == 3 {
                last_sum = sliding_window.iter().sum::<i32>();
            }
        }
    }

    println!("Part 2: {}", count);
}

fn main() {
    let lines_result = read_lines("../inputs/day1");
    if let Ok(lines) = lines_result {
        part1(lines.clone());

        part2(lines.clone())
    } else {
        println!("Something went wrong");
    }
}
