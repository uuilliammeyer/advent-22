use std::cmp;
use advent22::read_lines;
use advent22::TESTDIR;

fn main() {
    let mut best = 0;
    let mut curr = 0;
    // find (assuming it exits)
    if let Ok(lines) = read_lines(TESTDIR.to_owned() +"/day1/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(i) = line {
                // println!("{}", i);
                if !(i.is_empty()) {
                    let line_as_int: i32 = i.parse().unwrap(); 
                    curr += line_as_int;
                }
                else {
                    best = cmp::max(best, curr);
                    curr = 0;
                }

            }
        }
        // ensure that last value isn't max
        best = cmp::max(best, curr); 
    }
    println!("max calorioes: {}", best);
}