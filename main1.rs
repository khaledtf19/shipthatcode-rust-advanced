use std::io::{self, BufRead};
use std::rc::Rc;

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    let nums: Vec<i32> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // TODO: move `nums` into an Rc.
    // TODO: create two more handles to it.
    // TODO: print "count: " and the strong count.
    // TODO: print "sum: " and the sum of the shared vec.
    let r1 = Rc::new(nums);
    let _ = Rc::clone(&r1);
    let r3 = Rc::clone(&r1);

    println!("count: {}", Rc::strong_count(&r1));
    println!("sum: {}", r3.iter().fold(0, |sum, prev| { sum + prev }));
}
