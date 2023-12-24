use std::{vec, char};
use advent_of_code_2022::read_file_input;

fn main(){
    let file = read_file_input("05.txt");
    print!("Day 5: Part 1:");
    part1(&file);
    print!(" Part 2: ");
    part2(&file);
}

fn part1(input: &str){
    let (crates, orders) = input.split_once("\r\n\r\n").unwrap();
    let mut crates_list = parse_crates(crates);
    let orders = parse_orders(orders);

    for i in orders{
        for j in 0..i.0{
            let removed = crates_list[i.1].pop().unwrap();
            crates_list[i.2].push(removed)
        }
    }

    let mut answers: Vec<char> = Vec::new();
    for mut i in crates_list{
        let top = i.pop();
        match top {
            Some(c) => { answers.push(c); }
            None => {}
        }
    }

    for i in answers{
        print!("{}", i);
    }
}

fn part2(input: &str){
    let (crates, orders) = input.split_once("\r\n\r\n").unwrap();
    let mut crates_list = parse_crates(crates);
    let orders = parse_orders(orders);

    for i in orders{
        let mut removed_list: Vec<char> = Vec::new();
        for j in 0..i.0{
            removed_list.push(crates_list[i.1].pop().unwrap());
        }
        removed_list.reverse();
        crates_list[i.2].append(&mut removed_list)
    }

    let mut answers: Vec<char> = Vec::new();
    for mut i in crates_list{
        let top = i.pop();
        match top {
            Some(c) => { answers.push(c); }
            None => {}
        }
    }

    for i in answers{
        print!("{}", i);
    }
}


fn parse_crates(crates: &str) -> Vec<Vec<char>>{
    let crates_len = crates.lines().rev().next().unwrap().len() +1;
    let mut crates_list: Vec<Vec<char>> = vec![Vec::new(); crates_len/4];
    crates.lines().rev().filter(|line| line.contains("[")).for_each(|l| {
        let cvec : Vec<char> = l.chars().collect();
        for i in 0..crates_len / 4 {
            let content = cvec[i*4+1];
            if content != ' ' {
                crates_list[i].push(content);
            }
        }
    });

    crates_list
}

fn parse_orders(orders: &str) -> Vec<(usize, usize, usize)>{
    orders.lines().map(|line|{
        let instr: Vec<&str> = line.split_whitespace().collect();
        (instr[1].parse::<usize>().unwrap(),
        instr[3].parse::<usize>().unwrap() - 1, 
        instr[5].parse::<usize>().unwrap() - 1)
    }).collect()
}
