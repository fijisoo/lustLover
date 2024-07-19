use std::cell::RefCell;
use std::rc::Rc;

pub fn test() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}