extern crate regex;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn read_lines<P>(filename: P) -> Result<Vec<String>, io::Error>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    io::BufReader::new(file).lines().collect::<Result<Vec<String>, io::Error>>()
}

fn part1(lines: Vec<String>) {
    let re = Regex::new(r"^(forward|up|down) (\d+)$").unwrap();
    let mut forward = 0;
    let mut depth = 0;

    for line in lines {
        for cap in re.captures_iter(&line) {
            if let Ok(num) = &cap[2].parse::<i32>() {
                if &cap[1] == "forward" {
                    forward += num;
                } else if &cap[1] == "down" {
                    depth += num;
                } else if &cap[1] == "up" {
                    depth -= num;
                }
            }
        }
    }

    println!("Part 1: {}", forward * depth);
}

fn part2(lines: Vec<String>) {
    let re = Regex::new(r"^(forward|up|down) (\d+)$").unwrap();
    let mut forward = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in lines {
        for cap in re.captures_iter(&line) {
            if let Ok(num) = &cap[2].parse::<i32>() {
                if &cap[1] == "forward" {
                    forward += num;
                    depth += aim * num;
                } else if &cap[1] == "down" {
                    aim += num;
                } else if &cap[1] == "up" {
                    aim -= num;
                }
            }
        }
    }

    println!("Part 2: {}", forward * depth);
}

fn main() {
    let lines_result = read_lines("../inputs/day2");
    if let Ok(lines) = lines_result {
        part1(lines.clone());

        part2(lines.clone())
    } else {
        println!("Something went wrong");
    }
}
