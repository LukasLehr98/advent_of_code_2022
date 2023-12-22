use advent_of_code_2022::read_file_input;
use num::traits::ops::overflowing;

fn main(){
    let file = read_file_input("03.txt");


    println!("{}, {}", part1(&file), part2(&file));
}

fn part1(input : &str) -> u32{
    input.lines().map(|l|{
        let (f, s) = l.split_at(l.len() / 2);
        let mut overlap: char = ' ';
        for i in f.chars(){
            for j in s.chars(){
                if i == j { 
                    overlap = i; 
                    break;
                }
            }
        }
        if overlap.is_uppercase(){
            (overlap as u32) - 38
        } else {
            (overlap as u32) - 96
        }
    }).sum()
}

fn part2(input: &str) -> u32{
    let mut lines = input.lines().collect::<Vec<&str>>();
    let mut total = 0;
    let mut i = 0;

    loop {
        if i >= lines.len() { break; }

        let mut overlap = 0;
        let group = &lines[i..=i+2];
        i += 3;
        
        for i in group[0].chars(){
            for j in group[1].chars(){
                for k in group[2].chars(){
                    if i == j && j == k{
                        overlap = get_value_from_char(i);
                    }
                }
            }
        }
        
        total += overlap
    }
    total
}



fn get_value_from_char(c: char)-> u32{
    if c.is_uppercase(){
        (c as u32) - 38
    } else {
        (c as u32) - 96
    }
}