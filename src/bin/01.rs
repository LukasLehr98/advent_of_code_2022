use advent_of_code_2022::read_file_input;
use itertools::Itertools;

fn main(){
    let file = read_file_input("01.txt");

    let solve = file.split("\r\n\r\n").map(|f| {
        f.lines().map(|e| {
            e.parse::<u32>().unwrap()
        }).sum::<u32>()
    }).sorted().rev().collect::<Vec<_>>();
    
    let solution : (u32, u32) = (solve[0], solve[0..3].iter().sum());
    println!("Day 1: Part 1: {}, Part 2: {}", solution.0, solution.1);
}