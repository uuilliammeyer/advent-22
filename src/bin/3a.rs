use advent22::read_lines;
use advent22::TESTDIR;
use std::collections::HashSet;
use std::iter::FromIterator;
fn main() {
    let mut repeats: Vec<char> = Vec::new();
    // find (assuming it exits)
    if let Ok(lines) = read_lines(TESTDIR.to_owned() + "/day3/input.txt") {
        for line in lines {
            if let Ok(i) = line {
                if !(i.is_empty()) {
                    let ln:String = i.parse().unwrap(); 
                    let first_half:String = ln[..ln.len()/2].to_string(); 
                    let second_half:String= ln[ln.len()/2..].to_string();
                    let first_half_char_vec: Vec<char> = first_half.chars().collect();
                    let second_half_char_vec: Vec<char> = second_half.chars().collect();
                    
                    let fist_half_set:HashSet<char> = HashSet::from_iter(first_half_char_vec);
                    let second_half_set:HashSet<char> = HashSet::from_iter(second_half_char_vec);                    

                    // should only be one char but let's be sure
                    for x in fist_half_set.intersection(&second_half_set) {
                        let x_prime = x.to_owned();
                        repeats.push(x_prime);
                    }
                }
            }
        }
    }
    let mut sum:u32 = 0;
    for c in repeats {
        sum += get_char_value(c);
    }
    println!("{}", sum);
}

pub fn get_char_value(c: char) -> u32 {
    if c.is_ascii_uppercase() {
        return c as u32 - 38;
    }
    else {
        return c as u32 - 96;
    }
}

