fn main() {
    println!("Hello, world!");
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}

use std::collections::HashMap;

pub fn foo() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

use std::fmt;
use std::io;

pub fn function1() -> fmt::Result {
    Ok(())
}

pub fn function2() -> io::Result<()> {
    Ok(())
}

use std::io::Result as IoResult;

pub fn function3() -> IoResult<()> {
    Ok(())
}
