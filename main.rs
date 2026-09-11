use std::io::{self, BufRead};

// TODO: fn longer<'a>(a: &'a str, b: &'a str) -> &'a str
//       returns the longer of the two; `a` on a tie.
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if b.len() > a.len() {
        return b;
    }
    return a;
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let a = lines.next().unwrap().unwrap();
    let b = lines.next().unwrap().unwrap();

    // TODO: print the result of calling longer on a and b.
    println!("{}", longer(&a, &b));
}
