use advent22::read_lines;
use advent22::TESTDIR;

fn main() {
    let mut top_three = Vec::new();
    let mut curr = 0;
    // find (assuming it exits)
    if let Ok(lines) = read_lines(TESTDIR.to_owned() + "/day1/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(i) = line {
                // println!("{}", i);
                if !(i.is_empty()) {
                    let line_as_int: i32 = i.parse().unwrap(); 
                    curr += line_as_int;
                }
                else {
                    top_three.push(curr);
                    curr = 0;
                }

            }
        }
        // ensure that last value isn't max
        top_three.push(curr);
        top_three.sort();
        top_three = top_three[top_three.len()-3..top_three.len()].to_vec();
    }
    println!("Top three values: {:?}", top_three);
    let sum_top_three: i32 = top_three.iter().sum();
    println!("max calorioes: {}", sum_top_three);
}