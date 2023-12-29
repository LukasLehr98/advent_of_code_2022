use advent_of_code_2022::read_file_input;
use hashbrown::HashSet;
use itertools::Itertools;

fn main(){
    let file = read_file_input("06.txt");
    println!("Day 6: Part 1: {}, Part 2: {}", solve(&file, 4), solve(&file, 14));
}

fn solve(input : &str, len: usize) -> usize{
    let split = input.chars().collect::<Vec<char>>();

    for i in len-1..split.len(){
        let mut set = HashSet::new();
        let mut present = false;
        for i in i-(len-1)..=i{
            if !set.insert(split[i]) { present = true }
        }
        if !present {
            return i+1;
        }
    }

    0
}