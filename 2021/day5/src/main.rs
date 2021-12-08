extern crate regex;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;
use std::cmp;

fn read_lines<P>(filename: P) -> Result<Vec<String>, io::Error>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    io::BufReader::new(file).lines().collect::<Result<Vec<String>, io::Error>>()
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

fn part1(lines: Vec<String>) {
    let mut points: HashMap::<Point, u32> = HashMap::new();

    let mut output = 0;
    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    for line in lines {
        for captures in re.captures_iter(&line) {
            if let (Ok(x1), Ok(y1), Ok(x2), Ok(y2)) = (&captures[1].parse::<i32>(), &captures[2].parse::<i32>(), &captures[3].parse::<i32>(), &captures[4].parse::<i32>()) {
                if x1 == x2 {
                    let diff = (y1 - y2).abs();
                    let min = cmp::min(y1, y2);
                    (0..diff+1).map(|y| (*x1, min + y)).into_iter().for_each(|(x, y)| {
                        let point = Point::new(x, y);
                        let counter = points.entry(point).or_insert(0);
                        if *counter == 1 {
                            output += 1;
                        }
                        *counter += 1;
                    })
                } else if y1 == y2 {
                    let diff = (x1 - x2).abs();
                    let min = cmp::min(x1, x2);
                    (0..diff+1).map(|x| (min + x, *y1)).into_iter().for_each(|(x, y)| {
                        let point = Point::new(x, y);
                        let counter = points.entry(point).or_insert(0);
                        if *counter == 1 {
                            output += 1;
                        }
                        *counter += 1;
                    })
                }
            }
        }
    }

    println!("Part 1: {}", output);
}

fn part2(lines: Vec<String>) {
    let mut points: HashMap::<Point, u32> = HashMap::new();

    let mut output = 0;
    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    for line in lines {
        for captures in re.captures_iter(&line) {
            if let (Ok(x1), Ok(y1), Ok(x2), Ok(y2)) = (&captures[1].parse::<i32>(), &captures[2].parse::<i32>(), &captures[3].parse::<i32>(), &captures[4].parse::<i32>()) {
                if x1 == x2 {
                    let diff = (y1 - y2).abs();
                    let min = cmp::min(y1, y2);
                    (0..diff+1).map(|y| (*x1, min + y)).into_iter().for_each(|(x, y)| {
                        let point = Point::new(x, y);
                        let counter = points.entry(point).or_insert(0);
                        if *counter == 1 {
                            output += 1;
                        }
                        *counter += 1;
                    });
                } else if y1 == y2 {
                    let diff = (x1 - x2).abs();
                    let min = cmp::min(x1, x2);
                    (0..diff+1).map(|x| (min + x, *y1)).into_iter().for_each(|(x, y)| {
                        let point = Point::new(x, y);
                        let counter = points.entry(point).or_insert(0);
                        if *counter == 1 {
                            output += 1;
                        }
                        *counter += 1;
                    });
                } else if (x1 - x2).abs() == (y1 - y2).abs() {
                    let diff = (x1 - x2).abs();
                    let x_multiplier = if x1 > x2 { -1 } else { 1 };
                    let y_multiplier = if y1 > y2 { -1 } else { 1 };
                    (0..diff+1).map(|delta| (x1 + x_multiplier * delta, y1 + y_multiplier * delta)).into_iter().for_each(|(x, y)| {
                        let point = Point::new(x, y);
                        let counter = points.entry(point).or_insert(0);
                        if *counter == 1 {
                            output += 1;
                        }
                        *counter += 1;
                    });
                }
            }
        }
    }

    println!("Part 2: {}", output);
}

fn main() {
    let lines_result = read_lines("../inputs/day5");
    if let Ok(lines) = lines_result {
        part1(lines.clone());

        part2(lines.clone())
    } else {
        println!("Something went wrong");
    }
}
