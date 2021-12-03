use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> Result<Vec<String>, io::Error>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    io::BufReader::new(file).lines().collect::<Result<Vec<String>, io::Error>>()
}

fn part1(lines: Vec<String>) {
    let mut counts = vec!(0,0,0,0,0,0,0,0,0,0,0,0);

    for line in lines {
        for (index, digit) in line.chars().enumerate() {
            if let Some(num) = digit.to_digit(10) {
                if num == 1 {
                    counts[index] += 1;
                } else {
                    counts[index] -= 1;
                }
            }
        }
    }

    let mut gamma: Vec<char> = Vec::new();
    let mut epsilon: Vec<char> = Vec::new();
    for count in counts {
        if count > 0 {
            gamma.push('1');
            epsilon.push('0');
        } else if count < 0 {
            gamma.push('0');
            epsilon.push('1');
        } else {
            panic!("Unexpected even number of digits!");
        }
    }

    println!(
        "Part 1: {}", 
        isize::from_str_radix(&gamma.iter().collect::<String>(), 2).unwrap() * 
        isize::from_str_radix(&epsilon.iter().collect::<String>(), 2).unwrap()
    );
}

fn count_bits(bits: Vec<char>) -> i32 {
    return bits.iter().filter(|x| **x == '1').count() as i32 - bits.iter().filter(|x| **x == '0').count() as i32;
}

fn part2(lines: Vec<String>) {
    let mut oxygen_lines: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let mut index = 0;
    let mut digit;
    while oxygen_lines.len() > 1 {
        let bits = oxygen_lines.iter_mut().map(|line| line[index]).collect();
        let count = count_bits(bits);
        if count >= 0 {
            digit = '1';
        } else {
            digit = '0';
        }

        oxygen_lines = oxygen_lines.iter().filter(|line| line[index] == digit).cloned().collect();
        index += 1;
    }
    let oxygen = isize::from_str_radix(&oxygen_lines[0].iter().collect::<String>(), 2).unwrap();

    let mut co2_lines: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let mut index = 0;
    let mut digit;
    while co2_lines.len() > 1 {
        let bits = co2_lines.iter_mut().map(|line| line[index]).collect();
        let count = count_bits(bits);
        if count >= 0 {
            digit = '0';
        } else {
            digit = '1';
        }

        co2_lines = co2_lines.iter().filter(|line| line[index] == digit).cloned().collect();
        index += 1;
    }
    let co2 = isize::from_str_radix(&co2_lines[0].iter().collect::<String>(), 2).unwrap();

    println!("Part 2: {}", oxygen * co2);
}

fn main() {
    let lines_result = read_lines("../inputs/day3");
    if let Ok(lines) = lines_result {
        part1(lines.clone());

        part2(lines.clone())
    } else {
        println!("Something went wrong");
    }
}
