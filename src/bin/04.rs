use advent_of_code_2022::read_file_input;

fn main (){
    let file = read_file_input("04.txt");
    let solution = solve(&file);

    println!("Day 4, Part 1: {} Part 2 {}", solution.0, solution.1)
}

fn solve(input :&str) -> (u32, u32){
    let mut part1 = 0;
    let mut part2 = 0;
    input.lines().for_each(|l|{
        let (f, s) = l.split_once(",").unwrap();
        let (f1, f2) = f.split_once("-").unwrap();
        let (s1, s2) = s.split_once("-").unwrap();

        if ranges_overlap((f1.parse::<usize>().unwrap(), f2.parse::<usize>().unwrap()), (s1.parse::<usize>().unwrap(), s2.parse::<usize>().unwrap())){
            part2 += 1
        }


        if ranges_contain((f1.parse::<usize>().unwrap(), f2.parse::<usize>().unwrap()), (s1.parse::<usize>().unwrap(), s2.parse::<usize>().unwrap())){
            part1 += 1
        }
    });

    (part1, part2)
}

fn ranges_contain(first: (usize, usize), second: (usize, usize)) -> bool{
    if first.0 <= second.0 && second.1 <= first.1 { return true; }
    if first.0 >= second.0 && second.1 >= first.1 { return true; }
    false

}

fn ranges_overlap(first: (usize, usize), second: (usize, usize)) -> bool{
    if ranges_contain(first, second) { return true; }
    else if first.0 >= second.0 && first.0 <= second.1 || first.1 >= second.0 && first.1 <= second.1 { return true; }
    else { return false; }
}