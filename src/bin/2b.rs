use advent22::read_lines;
use advent22::TESTDIR;
use std::i32;

fn main() {
    let mut my_score: i32 = 0;
    // find (assuming it exits)
    if let Ok(lines) = read_lines(TESTDIR.to_owned() + "/day2/input.txt") {
        for line in lines {
            if let Ok(i) = line {
                if !(i.is_empty()) {
                    let ln:String = i.parse().unwrap(); 
                    my_score += line_value(&ln);
                }
            }
        }
    }
    println!("You're score: {}", my_score)
}

fn line_value(x: &str) -> i32 {
    match x {
        // Y is draw
        // X is lose
        // Z is win
        // throw rock and we need to lose, so throw scissors (3)
        "A X" => 3 + 0,
        // throw paper and we need to lose, so throw rock (1)
        "B X" => 1 + 0,
        // throw scissors and we need to lose, so throw paper (2)
        "C X" => 2 + 0,
        // throw rock and we need to draw, so throw rock (1)
        "A Y" => 1 + 3,
        // throw paper and we need to draw, so throw paper (2)
        "B Y" => 2 + 3,
        // throw scissors and we need to draw, so throw scissors (3)
        "C Y" => 3 + 3,
        // throw rock and we need to win, so throw paper (2)
        "A Z" => 6 + 2,
        // throw rock and we need to win, so throw scissors (3)
        "B Z" => 6 + 3,
        // throw scissors and we need to win, so throw rock (1)
        "C Z" => 6 + 1,
        _ => -100000
    }
}