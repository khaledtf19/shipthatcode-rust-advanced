use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    let n: i32 = line.trim().parse().unwrap();
    let raw: *const i32 = &n;

    // TODO: print the value behind `raw`, dereferencing it in an unsafe block.
    unsafe {
        let v = *raw;
        println!("{}", v);
    }
}
