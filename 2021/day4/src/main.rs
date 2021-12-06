use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn read_lines<P>(filename: P) -> Result<Vec<String>, io::Error>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    io::BufReader::new(file).lines().collect::<Result<Vec<String>, io::Error>>()
}

fn check_board(board: &Vec<Vec<&str>>) -> bool {
    for row in board.iter() {
        if row.iter().all(|x| *x == "Marked") {
            return true;
        }
    }

    if let Some(row) = board.iter().next() {
        for index in 0..row.len() {
            if board.iter().map(|x| x[index]).all(|x| x == "Marked") {
                return true;
            }
        }
    }

    false
}

fn part1(lines: Vec<String>) {
    let inputs = lines.iter().nth(0);

    let mut boards: Vec<Vec<Vec<&str>>> = Vec::new();

    while lines.iter().skip(boards.len() * 6 + 2).peekable().peek().is_some() {
        boards.push(lines.iter().skip(boards.len() * 6 + 2).take(5).map(|x| x.split_whitespace().collect()).collect::<Vec<Vec<&str>>>());
    }

    for input in inputs.unwrap().split(",") {
        for board_index in 0..boards.len() {
            for row_index in 0..5 {
                for col_index in 0..5 {
                    let item = boards[board_index][row_index][col_index];
                    if item == input {
                        boards[board_index][row_index][col_index] = &"Marked";

                        let found = check_board(&boards[board_index]);

                        if found {
                            let result = boards[board_index].iter().flatten().filter(|x| *x != &"Marked").map(|x| x.parse::<i32>().unwrap()).sum::<i32>();

                            println!("{:?}", boards[board_index]);
                            println!("Part 1: {}", result * input.parse::<i32>().unwrap());
                            return;
                        } else {
                            continue;
                        }
                    }
                }
            }
        }
    }
}

fn part2(lines: Vec<String>) {
    let inputs = lines.iter().nth(0);

    let mut boards: Vec<Vec<Vec<&str>>> = Vec::new();

    while lines.iter().skip(boards.len() * 6 + 2).peekable().peek().is_some() {
        boards.push(lines.iter().skip(boards.len() * 6 + 2).take(5).map(|x| x.split_whitespace().collect()).collect::<Vec<Vec<&str>>>());
    }

    let mut found_boards: HashSet<usize> = HashSet::new();
    let mut last_score = 0;
    for input in inputs.unwrap().split(",") {
        for board_index in 0..boards.len() {
            if !found_boards.contains(&board_index) {
                for row_index in 0..5 {
                    for col_index in 0..5 {
                        let item = boards[board_index][row_index][col_index];
                        if item == input {
                            boards[board_index][row_index][col_index] = &"Marked";

                            let found = check_board(&boards[board_index]);

                            if found {
                                found_boards.insert(board_index);
                                let result = boards[board_index].iter().flatten().filter(|x| *x != &"Marked").map(|x| x.parse::<i32>().unwrap()).sum::<i32>();

                                last_score = result * input.parse::<i32>().unwrap();
                            } else {
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Part 2: {}", last_score);
}

fn main() {
    let lines_result = read_lines("../inputs/day4");
    if let Ok(lines) = lines_result {
        part1(lines.clone());

        part2(lines.clone())
    } else {
        println!("Something went wrong");
    }
}
