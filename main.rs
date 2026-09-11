use std::{cell::RefCell, rc::Rc};

fn main() {
    let r = Rc::new(RefCell::new(0));

    for _ in 0..3 {
        let r1 = Rc::clone(&r);
        *r1.borrow_mut() += 1;
    }
    println!("{}", r.borrow());
}
