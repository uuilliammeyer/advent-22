use advent22::read_lines;
use advent22::TESTDIR;
use std::i32;

fn main() {
    let mut my_score: i32 = 0;
    // find (assuming it exits)
    if let Ok(lines) = read_lines(TESTDIR.to_owned() + "/day2/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(i) = line {
                // println!("{}", i);
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
        "A X" => 3 + 1,
        "B X" => 0 + 1,
        "C X" => 6 + 1,
        "A Y" => 6 + 2,
        "B Y" => 3 + 2,
        "C Y" => 0 + 2,
        "A Z" => 0 + 3,
        "B Z" => 6 + 3,
        "C Z" => 3 + 3,
        _ => unimplemented!("You shouldn't be here!")
    }
}