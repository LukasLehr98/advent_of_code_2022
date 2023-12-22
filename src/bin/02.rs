use advent_of_code_2022::read_file_input;

fn main(){
    let file = read_file_input("02.txt");
    println!("Day 2: Part 1: {}, Part 2: {}", part1(&file), part2(&file));
}

fn part1(input: &str) -> usize{
    input.lines().map(|game| {
        let game = play_game(get_input_score(game.split_once(" ").unwrap()));
        game
    }).sum()
}

fn part2(input: &str) -> usize{
    input.lines().map(|game| {
        let changed = match game.split_once(" ").unwrap(){
            ("A", "X") => ("A", "Z"),
            ("B", "X") => ("B", "X"),
            ("C", "X") => ("C", "Y"),
            // draw
            ("A", "Y") => ("A", "X"),
            ("B", "Y") => ("B", "Y"),
            ("C", "Y") => ("C", "Z"),
            // win
            ("A", "Z") => ("A", "Y"),
            ("B", "Z") => ("B", "Z"),
            ("C", "Z") => ("C", "X"),
            (x, y) => (x, y),
        };
        play_game(get_input_score(changed))
    }).sum()
}

fn get_input_score(input :(&str, &str)) -> (usize, usize){
    (match input.0 {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => 0
    },
    match input.1 {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
          _ => 0
    })
}

fn play_game(scores: (usize, usize)) -> usize{
    match scores {
        (opp, me) if opp == me => { 3 + me }, // Mirror
        (opp, me) if opp > me && opp - me == 2 => { me + 6} // Scissor vs Rock
        (opp, me) if me > opp && opp != 1  => { me + 6 } // Paper vs Scissor
        (opp, me) if me > opp && me-opp == 1 => { me + 6 } // Rock vs Paper
        (_opp, me) => { me } // Any loss
    }
}